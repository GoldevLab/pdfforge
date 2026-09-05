//! Multipart APIs + staged downloads.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{LazyLock, Mutex, OnceLock};
use std::time::{Duration, Instant};

use axum::extract::{ConnectInfo, Multipart, Path, Query};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use tokio::sync::Semaphore;

use crate::ops::{self, FileOut, MAX_UPLOAD_BYTES};
use crate::stage;

static SLOTS: LazyLock<Semaphore> = LazyLock::new(|| Semaphore::new(2));
const RATE_MAX: usize = 30;
const RATE_WINDOW: Duration = Duration::from_secs(60);

#[derive(Serialize)]
struct OkBody {
    ok: bool,
    original_bytes: usize,
    result_bytes: usize,
    pages: u32,
    format: String,
    mime: String,
    filename: String,
    url: String,
    text: Option<String>,
    extra: String,
}

#[derive(Serialize)]
struct ErrBody {
    ok: bool,
    error: String,
}

struct Parts {
    files: Vec<(Vec<u8>, String)>,
    fields: HashMap<String, String>,
}

async fn take_parts(multipart: &mut Multipart) -> Result<Parts, Response> {
    let mut parts = Parts {
        files: Vec::new(),
        fields: HashMap::new(),
    };
    loop {
        let field = match multipart.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) => {
                return Err(fail(
                    StatusCode::BAD_REQUEST,
                    format!("Upload failed: {e}"),
                ));
            }
        };
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" | "files" | "upload" => {
                let filename = field.file_name().unwrap_or("document.pdf").to_string();
                match field.bytes().await {
                    Ok(b) => parts.files.push((b.to_vec(), filename)),
                    Err(e) => {
                        return Err(fail(
                            StatusCode::BAD_REQUEST,
                            format!("Could not read the file: {e}"),
                        ));
                    }
                }
            }
            _ => {
                if let Ok(t) = field.text().await {
                    parts.fields.insert(name, t);
                }
            }
        }
    }
    Ok(parts)
}

fn field_str<'a>(parts: &'a Parts, key: &str) -> &'a str {
    parts.fields.get(key).map(|s| s.as_str()).unwrap_or("")
}

fn field_u32(parts: &Parts, key: &str) -> Option<u32> {
    parts.fields.get(key).and_then(|t| t.trim().parse().ok())
}

fn fail(status: StatusCode, error: String) -> Response {
    cors(status, Json(ErrBody { ok: false, error }))
}

pub async fn preflight() -> Response {
    cors(StatusCode::NO_CONTENT, ())
}

fn gate_files(files: Vec<(Vec<u8>, String)>, ip: &str, min: usize) -> Result<Vec<(Vec<u8>, String)>, Response> {
    if files.len() < min {
        return Err(fail(
            StatusCode::BAD_REQUEST,
            if min >= 2 {
                "Drop at least two PDFs.".into()
            } else {
                "Attach a PDF in the file field.".into()
            },
        ));
    }
    let mut total = 0usize;
    for (bytes, _) in &files {
        if bytes.is_empty() {
            return Err(fail(StatusCode::BAD_REQUEST, "Empty file.".into()));
        }
        if bytes.len() > MAX_UPLOAD_BYTES {
            return Err(fail(
                StatusCode::PAYLOAD_TOO_LARGE,
                "A file is over 20 MB.".into(),
            ));
        }
        total = total.saturating_add(bytes.len());
        if let Err(e) = ops::reject_unsupported(bytes) {
            return Err(fail(StatusCode::UNPROCESSABLE_ENTITY, e));
        }
    }
    if total > MAX_UPLOAD_BYTES * 2 {
        return Err(fail(
            StatusCode::PAYLOAD_TOO_LARGE,
            "Those files together are too large.".into(),
        ));
    }
    if !rate_allow(ip) {
        return Err(fail(
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests. Wait a minute and try again.".into(),
        ));
    }
    Ok(files)
}

fn gate_images(files: Vec<(Vec<u8>, String)>, ip: &str) -> Result<Vec<(Vec<u8>, String)>, Response> {
    if files.is_empty() {
        return Err(fail(
            StatusCode::BAD_REQUEST,
            "Drop at least one JPG or PNG.".into(),
        ));
    }
    if files.len() > crate::imgpdf::MAX_IMAGE_FILES {
        return Err(fail(
            StatusCode::BAD_REQUEST,
            format!("Convert at most {} images at a time.", crate::imgpdf::MAX_IMAGE_FILES),
        ));
    }
    let mut total = 0usize;
    for (bytes, _) in &files {
        if bytes.is_empty() {
            return Err(fail(StatusCode::BAD_REQUEST, "Empty file.".into()));
        }
        if bytes.len() > MAX_UPLOAD_BYTES {
            return Err(fail(
                StatusCode::PAYLOAD_TOO_LARGE,
                "A file is over 20 MB.".into(),
            ));
        }
        total = total.saturating_add(bytes.len());
        if let Err(e) = crate::imgpdf::reject_non_image(bytes) {
            return Err(fail(StatusCode::UNPROCESSABLE_ENTITY, e));
        }
    }
    if total > MAX_UPLOAD_BYTES * 2 {
        return Err(fail(
            StatusCode::PAYLOAD_TOO_LARGE,
            "Those files together are too large.".into(),
        ));
    }
    if !rate_allow(ip) {
        return Err(fail(
            StatusCode::TOO_MANY_REQUESTS,
            "Too many requests. Wait a minute and try again.".into(),
        ));
    }
    Ok(files)
}

async fn run_cpu<T, E>(job: impl FnOnce() -> Result<T, E> + Send + 'static) -> Result<T, Response>
where
    T: Send + 'static,
    E: ToString + Send + 'static,
{
    let Ok(permit) = tokio::time::timeout(Duration::from_secs(8), SLOTS.acquire()).await else {
        return Err(fail(
            StatusCode::SERVICE_UNAVAILABLE,
            "The server is busy. Try again in a few seconds.".into(),
        ));
    };
    let permit = match permit {
        Ok(p) => p,
        Err(_) => {
            return Err(fail(
                StatusCode::SERVICE_UNAVAILABLE,
                "The server is busy. Try again in a few seconds.".into(),
            ));
        }
    };
    let work = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        job()
    });
    match tokio::time::timeout(Duration::from_secs(45), work).await {
        Err(_) => Err(fail(
            StatusCode::GATEWAY_TIMEOUT,
            "That PDF took too long. Try fewer pages.".into(),
        )),
        Ok(Err(_)) => Err(fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Processing failed.".into(),
        )),
        Ok(Ok(Err(e))) => Err(fail(StatusCode::UNPROCESSABLE_ENTITY, e.to_string())),
        Ok(Ok(Ok(r))) => Ok(r),
    }
}

fn staged(out: FileOut, stem: &str, original_bytes: usize, include_text: bool) -> Response {
    let filename = format!("{stem}.{}", out.ext);
    let text = if include_text && out.mime.starts_with("text/plain") {
        Some(String::from_utf8_lossy(&out.bytes).into_owned())
    } else {
        None
    };
    let mime = if out.mime.starts_with("text/plain") {
        "text/plain; charset=utf-8"
    } else {
        out.mime
    };
    let id = match stage::put(out.bytes.clone(), mime, &filename) {
        Ok(id) => id,
        Err(_) => {
            return fail(
                StatusCode::INSUFFICIENT_STORAGE,
                "Could not stage the download.".into(),
            );
        }
    };
    cors(
        StatusCode::OK,
        Json(OkBody {
            ok: true,
            original_bytes,
            result_bytes: out.bytes.len(),
            pages: out.pages_in,
            format: out.ext.into(),
            mime: mime.into(),
            filename,
            url: format!("/d/{id}"),
            text,
            extra: out.extra,
        }),
    )
}

pub async fn compress_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let ip = client_ip(&headers, Some(addr));
    let parts = match take_parts(&mut multipart).await {
        Ok(p) => p,
        Err(r) => return r,
    };
    let files = match gate_files(parts.files, &ip, 1) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some((bytes, name)) = files.into_iter().next() else {
        return fail(StatusCode::BAD_REQUEST, "Attach a PDF in the file field.".into());
    };
    let orig = bytes.len();
    let stem = format!("{}-compressed", ops::stem_filename(&name));
    match run_cpu(move || ops::compress(&bytes)).await {
        Ok(out) => staged(out, &stem, orig, false),
        Err(r) => r,
    }
}

pub async fn merge_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let ip = client_ip(&headers, Some(addr));
    let parts = match take_parts(&mut multipart).await {
        Ok(p) => p,
        Err(r) => return r,
    };
    let files = match gate_files(parts.files, &ip, 2) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let orig: usize = files.iter().map(|(b, _)| b.len()).sum();
    match run_cpu(move || ops::merge(&files)).await {
        Ok(out) => staged(out, "merged", orig, false),
        Err(r) => r,
    }
}

pub async fn from_jpg_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let ip = client_ip(&headers, Some(addr));
    let parts = match take_parts(&mut multipart).await {
        Ok(p) => p,
        Err(r) => return r,
    };
    let files = match gate_images(parts.files, &ip) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let orig: usize = files.iter().map(|(b, _)| b.len()).sum();
    match run_cpu(move || crate::imgpdf::from_images(&files)).await {
        Ok(out) => staged(out, "images", orig, false),
        Err(r) => r,
    }
}

pub async fn split_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let ip = client_ip(&headers, Some(addr));
    let parts = match take_parts(&mut multipart).await {
        Ok(p) => p,
        Err(r) => return r,
    };
    let range = field_str(&parts, "range").to_string();
    let each = field_str(&parts, "mode") == "each";
    let files = match gate_files(parts.files, &ip, 1) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some((bytes, name)) = files.into_iter().next() else {
        return fail(StatusCode::BAD_REQUEST, "Attach a PDF in the file field.".into());
    };
    let orig = bytes.len();
    let stem = format!("{}-split", ops::stem_filename(&name));
    match run_cpu(move || ops::split(&bytes, &range, each)).await {
        Ok(out) => staged(out, &stem, orig, false),
        Err(r) => r,
    }
}

pub async fn jpg_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let ip = client_ip(&headers, Some(addr));
    let parts = match take_parts(&mut multipart).await {
        Ok(p) => p,
        Err(r) => return r,
    };
    let range = field_str(&parts, "range").to_string();
    let dpi = field_u32(&parts, "dpi").unwrap_or(110);
    let files = match gate_files(parts.files, &ip, 1) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some((bytes, name)) = files.into_iter().next() else {
        return fail(StatusCode::BAD_REQUEST, "Attach a PDF in the file field.".into());
    };
    let orig = bytes.len();
    let stem = format!("{}-pages", ops::stem_filename(&name));
    match run_cpu(move || ops::to_jpg(&bytes, &range, dpi)).await {
        Ok(out) => staged(out, &stem, orig, false),
        Err(r) => r,
    }
}

pub async fn text_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let ip = client_ip(&headers, Some(addr));
    let parts = match take_parts(&mut multipart).await {
        Ok(p) => p,
        Err(r) => return r,
    };
    let range = field_str(&parts, "range").to_string();
    let files = match gate_files(parts.files, &ip, 1) {
        Ok(v) => v,
        Err(r) => return r,
    };
    let Some((bytes, name)) = files.into_iter().next() else {
        return fail(StatusCode::BAD_REQUEST, "Attach a PDF in the file field.".into());
    };
    let orig = bytes.len();
    let stem = format!("{}-text", ops::stem_filename(&name));
    match run_cpu(move || ops::to_text(&bytes, &range)).await {
        Ok(out) => staged(out, &stem, orig, true),
        Err(r) => r,
    }
}

#[derive(serde::Deserialize)]
pub struct DlQuery {
    pub dl: Option<String>,
}

pub async fn download(Path(id): Path<String>, Query(q): Query<DlQuery>) -> Response {
    if !stage::is_id(&id) {
        return fail(StatusCode::NOT_FOUND, "Unknown download.".into());
    }
    let Some(file) = stage::get(&id) else {
        return fail(StatusCode::NOT_FOUND, "That download expired.".into());
    };
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&file.mime).unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, max-age=300"),
    );
    let kind = if q.dl.as_deref() == Some("1") {
        "attachment"
    } else {
        "inline"
    };
    let safe = file
        .filename
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') { c } else { '_' })
        .collect::<String>();
    if let Ok(cd) = HeaderValue::from_str(&format!("{kind}; filename=\"{safe}\"")) {
        headers.insert(header::CONTENT_DISPOSITION, cd);
    }
    (StatusCode::OK, headers, file.bytes.to_vec()).into_response()
}

fn cors<T: IntoResponse>(status: StatusCode, body: T) -> Response {
    let mut res = (status, body).into_response();
    let h = res.headers_mut();
    h.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    h.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("POST, OPTIONS"),
    );
    h.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("content-type, x-resuma-csrf"),
    );
    h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    res
}

fn client_ip(headers: &HeaderMap, peer: Option<SocketAddr>) -> String {
    headers
        .get("fly-client-ip")
        .and_then(|v| v.to_str().ok())
        .filter(|s| s.parse::<std::net::IpAddr>().is_ok())
        .map(|s| s.to_string())
        .or_else(|| {
            headers
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.split(',').next())
                .map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|| {
            peer.map(|a| a.ip().to_string())
                .unwrap_or_else(|| "0.0.0.0".into())
        })
}

fn rate_allow(ip: &str) -> bool {
    static MAP: OnceLock<Mutex<HashMap<String, (Instant, usize)>>> = OnceLock::new();
    let map = MAP.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(mut g) = map.lock() else {
        return true;
    };
    let now = Instant::now();
    g.retain(|_, (at, _)| now.duration_since(*at) < RATE_WINDOW);
    let e = g.entry(ip.to_string()).or_insert((now, 0));
    if now.duration_since(e.0) >= RATE_WINDOW {
        *e = (now, 0);
    }
    if e.1 >= RATE_MAX {
        return false;
    }
    e.1 += 1;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn prefers_fly_client_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("fly-client-ip", HeaderValue::from_static("203.0.113.9"));
        headers.insert("x-forwarded-for", HeaderValue::from_static("1.2.3.4"));
        let peer = "127.0.0.1:1".parse().unwrap();
        assert_eq!(client_ip(&headers, Some(peer)), "203.0.113.9");
    }
}
