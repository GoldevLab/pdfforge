//! One JPEG/PNG per page, fit on A4. PNG is re-encoded as JPEG.

use std::io::Cursor;

use image::ImageFormat;

use crate::ops::FileOut;

pub const MAX_IMAGE_FILES: usize = 12;

pub fn from_images(files: &[(Vec<u8>, String)]) -> Result<FileOut, String> {
    if files.is_empty() {
        return Err("Drop at least one JPG or PNG.".into());
    }
    if files.len() > MAX_IMAGE_FILES {
        return Err(format!("Convert at most {MAX_IMAGE_FILES} images at a time."));
    }
    let mut pages = Vec::new();
    for (bytes, name) in files {
        pages.push(to_jpeg_page(bytes, name)?);
    }
    let pdf = assemble_jpeg_pdf(&pages)?;
    Ok(FileOut {
        bytes: pdf,
        mime: "application/pdf",
        ext: "pdf",
        pages_in: pages.len() as u32,
        extra: format!("{} image{}", pages.len(), if pages.len() == 1 { "" } else { "s" }),
    })
}

pub fn reject_non_image(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 8 {
        return Err("That file is too small to be an image.".into());
    }
    if bytes.starts_with(&[0xFF, 0xD8]) || bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Ok(());
    }
    Err("Use a JPG or PNG.".into())
}

fn to_jpeg_page(bytes: &[u8], name: &str) -> Result<(u32, u32, Vec<u8>), String> {
    reject_non_image(bytes)?;
    let img = image::load_from_memory(bytes)
        .map_err(|e| format!("Could not read {name}: {e}"))?
        .to_rgb8();
    let w = img.width();
    let h = img.height();
    if w == 0 || h == 0 {
        return Err("That image has no pixels.".into());
    }
    if w > 12_000 || h > 12_000 {
        return Err("That image is too large (max 12,000 px).".into());
    }
    let mut jpeg = Vec::new();
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut Cursor::new(&mut jpeg), ImageFormat::Jpeg)
        .map_err(|e| format!("JPEG encode failed: {e}"))?;
    Ok((w, h, jpeg))
}

fn assemble_jpeg_pdf(pages: &[(u32, u32, Vec<u8>)]) -> Result<Vec<u8>, String> {
    const PW: f32 = 595.0;
    const PH: f32 = 842.0;
    const M: f32 = 24.0;
    let n = pages.len();
    if n == 0 {
        return Err("No pages.".into());
    }
    let mut objs: Vec<Vec<u8>> = Vec::with_capacity(2 + n * 3);
    objs.push(b"<< /Type /Catalog /Pages 2 0 R >>".to_vec());
    let mut kids = String::from("[");
    for i in 0..n {
        let page_id = 3 + i * 3;
        if i > 0 {
            kids.push(' ');
        }
        kids.push_str(&format!("{page_id} 0 R"));
    }
    kids.push(']');
    objs.push(format!("<< /Type /Pages /Kids {kids} /Count {n} >>").into_bytes());

    for (i, (w, h, jpeg)) in pages.iter().enumerate() {
        let page_id = 3 + i * 3;
        let img_id = page_id + 1;
        let content_id = page_id + 2;
        let max_w = PW - 2.0 * M;
        let max_h = PH - 2.0 * M;
        let scale = (max_w / *w as f32).min(max_h / *h as f32);
        let dw = *w as f32 * scale;
        let dh = *h as f32 * scale;
        let x = (PW - dw) / 2.0;
        let y = (PH - dh) / 2.0;
        objs.push(
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {PW} {PH}] \
                 /Resources << /XObject << /Im0 {img_id} 0 R >> >> /Contents {content_id} 0 R >>"
            )
            .into_bytes(),
        );
        let mut img_dict = format!(
            "<< /Type /XObject /Subtype /Image /Width {w} /Height {h} \
             /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode /Length {} >>\nstream\n",
            jpeg.len()
        )
        .into_bytes();
        img_dict.extend_from_slice(jpeg);
        img_dict.extend_from_slice(b"\nendstream");
        objs.push(img_dict);
        let content = format!("q {dw:.2} 0 0 {dh:.2} {x:.2} {y:.2} cm /Im0 Do Q");
        objs.push(
            format!(
                "<< /Length {} >>\nstream\n{content}\nendstream",
                content.len()
            )
            .into_bytes(),
        );
    }

    let mut out = Vec::from(&b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n"[..]);
    let mut offsets = Vec::with_capacity(objs.len() + 1);
    offsets.push(0u32);
    for (i, body) in objs.iter().enumerate() {
        offsets.push(out.len() as u32);
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref_at = out.len();
    let count = objs.len() + 1;
    out.extend_from_slice(format!("xref\n0 {count}\n").as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for off in offsets.iter().skip(1) {
        out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!("trailer\n<< /Size {count} /Root 1 0 R >>\nstartxref\n{xref_at}\n%%EOF\n").as_bytes(),
    );
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    fn tiny_jpg() -> Vec<u8> {
        let mut img = RgbImage::new(16, 10);
        for p in img.pixels_mut() {
            *p = Rgb([200, 40, 40]);
        }
        let mut jpeg = Vec::new();
        image::DynamicImage::ImageRgb8(img)
            .write_to(&mut Cursor::new(&mut jpeg), ImageFormat::Jpeg)
            .expect("jpeg");
        jpeg
    }

    #[test]
    fn jpg_becomes_pdf() {
        let bytes = tiny_jpg();
        let out = from_images(&[(bytes, "a.jpg".into())]).expect("pdf");
        assert!(out.bytes.starts_with(b"%PDF"));
        assert_eq!(out.pages_in, 1);
        assert_eq!(out.ext, "pdf");
    }

    #[test]
    fn rejects_pdf_bytes() {
        assert!(reject_non_image(b"%PDF-1.4xxxx").is_err());
    }
}
