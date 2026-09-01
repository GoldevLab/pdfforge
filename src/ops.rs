//! PDF jobs via pdq (pure Rust: lopdf + hayro). Temp files, then bytes back.

use std::fs;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use image::ImageFormat;
use pdq::merge::MergeInput;
use pdq::range::PageRangeGroup;
use pdq::render::RenderOptions;
use pdq::split::{SplitOutput, SplitPagesOptions};
use pdq::text::ExtractTextOptions;
use tempfile::TempDir;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub const MAX_UPLOAD_BYTES: usize = 20 * 1024 * 1024;
pub const MAX_MERGE_FILES: usize = 8;
pub const MAX_RENDER_PAGES: u32 = 12;

pub struct FileOut {
    pub bytes: Vec<u8>,
    pub mime: &'static str,
    pub ext: &'static str,
    pub pages_in: u32,
    pub extra: String,
}

pub fn reject_unsupported(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 5 || !bytes.starts_with(b"%PDF") {
        return Err("That file is not a PDF.".into());
    }
    Ok(())
}

pub fn stem_filename(name: &str) -> String {
    let stem = Path::new(name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("document");
    let cleaned: String = stem
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .take(48)
        .collect();
    if cleaned.is_empty() {
        "document".into()
    } else {
        cleaned
    }
}

fn workdir() -> Result<TempDir, String> {
    TempDir::new().map_err(|e| format!("Could not open a work folder: {e}"))
}

fn write_pdf(dir: &Path, name: &str, bytes: &[u8]) -> Result<PathBuf, String> {
    reject_unsupported(bytes)?;
    let path = dir.join(name);
    fs::write(&path, bytes).map_err(|e| format!("Could not write a temp PDF: {e}"))?;
    Ok(path)
}

fn read_file(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("Could not read the result: {e}"))
}

fn page_count(path: &Path) -> Result<u32, String> {
    pdq::page_count(path)
        .map(|n| n as u32)
        .map_err(|e| format!("Could not read page count: {e}"))
}

fn parse_range(raw: &str) -> Result<PageRangeGroup, String> {
    let s = raw.trim();
    let s = if s.is_empty() { "1-z" } else { s };
    PageRangeGroup::parse(s).map_err(|e| format!("Invalid page range ({s}): {e}"))
}

/// Whole-document rewrite: drop unreferenced objects (pdq `1-z` split).
pub fn compress(bytes: &[u8]) -> Result<FileOut, String> {
    let dir = workdir()?;
    let input = write_pdf(dir.path(), "in.pdf", bytes)?;
    let pages_in = page_count(&input)?;
    let out = dir.path().join("out.pdf");
    pdq::split(
        &input,
        &[SplitOutput {
            range: parse_range("1-z")?,
            path: out.clone(),
        }],
    )
    .map_err(|e| format!("Compress failed: {e}"))?;
    let out_bytes = read_file(&out)?;
    Ok(FileOut {
        bytes: out_bytes,
        mime: "application/pdf",
        ext: "pdf",
        pages_in,
        extra: String::new(),
    })
}

pub fn merge(files: &[(Vec<u8>, String)]) -> Result<FileOut, String> {
    if files.len() < 2 {
        return Err("Drop at least two PDFs to merge.".into());
    }
    if files.len() > MAX_MERGE_FILES {
        return Err(format!("Merge at most {MAX_MERGE_FILES} PDFs at a time."));
    }
    let dir = workdir()?;
    let mut inputs = Vec::new();
    let mut pages_in = 0u32;
    for (i, (bytes, _)) in files.iter().enumerate() {
        let path = write_pdf(dir.path(), &format!("in-{i}.pdf"), bytes)?;
        pages_in += page_count(&path)?;
        inputs.push(MergeInput::all(&path));
    }
    let out = dir.path().join("merged.pdf");
    pdq::merge(&inputs, &out).map_err(|e| format!("Merge failed: {e}"))?;
    Ok(FileOut {
        bytes: read_file(&out)?,
        mime: "application/pdf",
        ext: "pdf",
        pages_in,
        extra: format!("{} files", files.len()),
    })
}

pub fn split(bytes: &[u8], range: &str, each_page: bool) -> Result<FileOut, String> {
    let dir = workdir()?;
    let input = write_pdf(dir.path(), "in.pdf", bytes)?;
    let pages_in = page_count(&input)?;
    if each_page {
        if pages_in > 40 {
            return Err("Bursting every page is limited to 40 pages. Extract a range instead.".into());
        }
        let pattern = dir
            .path()
            .join("page-%d.pdf")
            .to_string_lossy()
            .into_owned();
        pdq::split_pages_with_options(
            &input,
            &pattern,
            &SplitPagesOptions {
                pages_per_file: 1,
                password: None,
            },
        )
        .map_err(|e| format!("Split failed: {e}"))?;
        let mut names: Vec<PathBuf> = fs::read_dir(dir.path())
            .map_err(|e| e.to_string())?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().and_then(|x| x.to_str()) == Some("pdf")
                    && p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with("page-"))
            })
            .collect();
        names.sort();
        if names.is_empty() {
            return Err("No pages came out of that split.".into());
        }
        if names.len() == 1 {
            return Ok(FileOut {
                bytes: read_file(&names[0])?,
                mime: "application/pdf",
                ext: "pdf",
                pages_in,
                extra: "1 page".into(),
            });
        }
        return zip_files(&names, "application/pdf").map(|bytes| FileOut {
            bytes,
            mime: "application/zip",
            ext: "zip",
            pages_in,
            extra: format!("{} files", names.len()),
        });
    }
    let out = dir.path().join("part.pdf");
    pdq::split(
        &input,
        &[SplitOutput {
            range: parse_range(range)?,
            path: out.clone(),
        }],
    )
    .map_err(|e| format!("Split failed: {e}"))?;
    Ok(FileOut {
        bytes: read_file(&out)?,
        mime: "application/pdf",
        ext: "pdf",
        pages_in,
        extra: range.trim().to_string(),
    })
}

pub fn to_jpg(bytes: &[u8], range: &str, dpi: u32) -> Result<FileOut, String> {
    let dpi = dpi.clamp(72, 150) as f32;
    let dir = workdir()?;
    let input = write_pdf(dir.path(), "in.pdf", bytes)?;
    let pages_in = page_count(&input)?;
    if pages_in > MAX_RENDER_PAGES && range.trim().is_empty() {
        return Err(format!(
            "This PDF has {pages_in} pages. Enter a range (for example 1-3). Max {MAX_RENDER_PAGES} pages per render."
        ));
    }
    let last = pages_in.min(MAX_RENDER_PAGES).max(1);
    let pages = if range.trim().is_empty() {
        parse_range(&format!("1-{last}"))?
    } else {
        parse_range(range)?
    };
    let pattern = dir
        .path()
        .join("page-%d.png")
        .to_string_lossy()
        .into_owned();
    pdq::render_pages(
        &input,
        &pattern,
        &RenderOptions {
            dpi,
            pages: Some(pages),
        },
    )
    .map_err(|e| format!("Render failed: {e}"))?;
    let mut pngs: Vec<PathBuf> = fs::read_dir(dir.path())
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("png"))
        .collect();
    pngs.sort();
    if pngs.is_empty() {
        return Err("No pages rendered. Try a simpler PDF or a smaller range.".into());
    }
    let jpgs: Vec<(String, Vec<u8>)> = pngs
        .iter()
        .map(|p| {
            let png = fs::read(p).map_err(|e| e.to_string())?;
            let img = image::load_from_memory(&png).map_err(|e| format!("Could not read a page image: {e}"))?;
            let mut jpg = Vec::new();
            img.to_rgb8()
                .write_to(&mut Cursor::new(&mut jpg), ImageFormat::Jpeg)
                .map_err(|e| format!("JPEG encode failed: {e}"))?;
            let name = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("page")
                .to_string()
                + ".jpg";
            Ok((name, jpg))
        })
        .collect::<Result<Vec<_>, String>>()?;
    if jpgs.len() == 1 {
        return Ok(FileOut {
            bytes: jpgs.into_iter().next().unwrap().1,
            mime: "image/jpeg",
            ext: "jpg",
            pages_in,
            extra: String::new(),
        });
    }
    zip_named(&jpgs).map(|bytes| FileOut {
        bytes,
        mime: "application/zip",
        ext: "zip",
        pages_in,
        extra: format!("{} pages", jpgs.len()),
    })
}

pub fn to_text(bytes: &[u8], range: &str) -> Result<FileOut, String> {
    let dir = workdir()?;
    let input = write_pdf(dir.path(), "in.pdf", bytes)?;
    let pages_in = page_count(&input)?;
    let pages = if range.trim().is_empty() {
        None
    } else {
        Some(parse_range(range)?)
    };
    let pages_out = pdq::extract_text(
        &input,
        &ExtractTextOptions {
            pages,
            password: None,
            annotations: false,
        },
    )
    .map_err(|e| format!("Text extract failed: {e}"))?;
    let mut body = String::new();
    for page in &pages_out {
        if !body.is_empty() {
            body.push_str("\n\n");
        }
        body.push_str(&format!("--- page {} ---\n", page.page));
        let mut runs = page.runs.clone();
        runs.sort_by(|a, b| {
            a.y.partial_cmp(&b.y)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal))
        });
        for run in runs {
            let t = run.text.trim();
            if !t.is_empty() {
                body.push_str(t);
                body.push('\n');
            }
        }
    }
    if body.trim().is_empty() {
        return Err("No selectable text in that PDF (it may be a scan).".into());
    }
    Ok(FileOut {
        bytes: body.into_bytes(),
        mime: "text/plain; charset=utf-8",
        ext: "txt",
        pages_in,
        extra: format!("{} pages of text", pages_out.len()),
    })
}

fn zip_files(paths: &[PathBuf], _inner: &str) -> Result<Vec<u8>, String> {
    let named: Vec<(String, Vec<u8>)> = paths
        .iter()
        .map(|p| {
            let name = p
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("page.pdf")
                .to_string();
            Ok((name, read_file(p)?))
        })
        .collect::<Result<_, String>>()?;
    zip_named(&named)
}

fn zip_named(files: &[(String, Vec<u8>)]) -> Result<Vec<u8>, String> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let opts = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, bytes) in files {
        zip.start_file(name, opts)
            .map_err(|e| format!("Zip failed: {e}"))?;
        zip.write_all(bytes).map_err(|e| format!("Zip failed: {e}"))?;
    }
    let cursor = zip.finish().map_err(|e| format!("Zip failed: {e}"))?;
    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/hello.pdf")).expect("hello.pdf")
    }

    #[test]
    fn rejects_non_pdf() {
        assert!(reject_unsupported(b"not a pdf").is_err());
        assert!(reject_unsupported(b"%PDF-1.4").is_ok());
    }

    #[test]
    fn compress_keeps_pdf() {
        let out = compress(&fixture()).expect("compress");
        assert!(out.bytes.starts_with(b"%PDF"));
        assert!(out.pages_in >= 1);
    }

    #[test]
    fn merge_two_copies() {
        let a = fixture();
        let b = a.clone();
        let out = merge(&[(a, "a.pdf".into()), (b, "b.pdf".into())]).expect("merge");
        assert!(out.bytes.starts_with(b"%PDF"));
        assert!(out.pages_in >= 2);
    }

    #[test]
    fn split_first_page() {
        let out = split(&fixture(), "1", false).expect("split");
        assert!(out.bytes.starts_with(b"%PDF"));
    }

    #[test]
    fn extract_text_mentions_hello() {
        let out = to_text(&fixture(), "").expect("text");
        let s = String::from_utf8_lossy(&out.bytes);
        assert!(
            s.to_ascii_lowercase().contains("hello")
                || s.contains("PDFForge")
                || s.contains("Page")
        );
    }

    #[test]
    fn split_each_page_zips_or_pdf() {
        let out = split(&fixture(), "", true).expect("burst");
        assert!(out.ext == "zip" || out.ext == "pdf");
        assert!(!out.bytes.is_empty());
    }

    #[test]
    fn to_jpg_default_range() {
        let out = to_jpg(&fixture(), "", 72).expect("jpg");
        assert!(out.ext == "jpg" || out.ext == "zip");
        assert!(!out.bytes.is_empty());
        if out.ext == "jpg" {
            assert_eq!(&out.bytes[..2], b"\xff\xd8");
        }
    }
}
