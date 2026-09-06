//! SEO landings (long-form) and compact /app shells.

use resuma::prelude::*;
use serde_json::json;

use crate::family::Tool;
use crate::tool;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    Seo,
    App,
}

impl Tool {
    fn title(self) -> &'static str {
        match self {
            Self::Compress => "Compress PDF online, free | PDFForge",
            Self::Merge => "Merge PDF files online, free | PDFForge",
            Self::Split => "Split a PDF by page range | PDFForge",
            Self::FromJpg => "Convert JPG to PDF online, free | PDFForge",
            Self::ToJpg => "Convert PDF to JPG online | PDFForge",
            Self::ToText => "Extract text from a PDF | PDFForge",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Compress => {
                "Rewrite a PDF and drop unused objects. No account. Uploads up to 20 MB."
            }
            Self::Merge => {
                "Combine PDFs into one file, in order. No account. Up to eight files, 20 MB each."
            }
            Self::Split => {
                "Keep a page range or burst every page into a zip. No account."
            }
            Self::FromJpg => {
                "Turn JPG or PNG images into a PDF. One image per page. No account. Up to 12 files, 20 MB each."
            }
            Self::ToJpg => {
                "Turn PDF pages into JPG. One image or a zip. Pure-Rust renderer, no account."
            }
            Self::ToText => {
                "Copy selectable text from a PDF to a .txt file. Not OCR. No account."
            }
        }
    }

    fn eyebrow(self) -> &'static str {
        match self {
            Self::Compress => "Smaller PDF",
            Self::Merge => "One file",
            Self::Split => "Pages out",
            Self::FromJpg => "Images in",
            Self::ToJpg => "Raster pages",
            Self::ToText => "Plain text",
        }
    }

    fn h1(self) -> &'static str {
        match self {
            Self::Compress => "Compress a PDF",
            Self::Merge => "Merge PDF files",
            Self::Split => "Split a PDF",
            Self::FromJpg => "Convert JPG to PDF",
            Self::ToJpg => "Convert PDF to JPG",
            Self::ToText => "Extract text from a PDF",
        }
    }

    fn lead(self) -> &'static str {
        match self {
            Self::Compress => {
                "We rewrite the document and drop objects nothing references. That shrinks many exports from Word or design tools. Embedded photos are not re-encoded like Distiller."
            }
            Self::Merge => {
                "Drop two or more PDFs. The first file in the picker is page 1 of the result. Bookmarks that only made sense in one file are not rebuilt."
            }
            Self::Split => {
                "Keep 1-3, 4-z, or the last page with r1. Burst mode writes one PDF per page and zips them when there is more than one."
            }
            Self::FromJpg => {
                "Drop photos or scans. Each file becomes one A4 page, centered. PNG is saved as JPEG inside the PDF. Order follows the picker."
            }
            Self::ToJpg => {
                "Hayro (pure Rust) paints each page. 110 DPI is the default. Leave the range blank to render the first 12 pages."
            }
            Self::ToText => {
                "We read the text layer, not pixels. A photographed invoice with no fonts will come back empty — use OCR elsewhere for that."
            }
        }
    }

    fn howto(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Compress => &[
                ("Drop the PDF", "Up to 20 MB. Encrypted files that only have an owner password usually open."),
                ("Rewrite", "Same pages, unused fonts and objects removed. Compare the sizes under the result."),
                ("Download", "The file stays available about 30 minutes. We do not keep a library of your documents."),
            ],
            Self::Merge => &[
                ("Pick several files", "Hold Ctrl or Shift in the file dialog, or drop a stack."),
                ("Order", "The picker order is the page order. Re-select if you need a different sequence."),
                ("Download one PDF", "Info metadata comes from the first file."),
            ],
            Self::Split => &[
                ("Drop one PDF", "Same 20 MB cap as the rest of PDFForge."),
                ("Range or burst", "A range stays one PDF. Burst (max 40 pages) becomes a zip of one-page files."),
                ("Download", "Open the result, then jump to compress or JPG if you still need those."),
            ],
            Self::FromJpg => &[
                ("Drop JPG or PNG", "Up to 12 files, 20 MB each. Hold Ctrl or Shift to pick a stack."),
                ("One page each", "We fit the image on A4. We do not OCR or vectorize."),
                ("Download the PDF", "Then compress or merge if you still need those."),
            ],
            Self::ToJpg => &[
                ("Drop the PDF", "Vector pages and simple scans work best."),
                ("Pages and DPI", "72–150 DPI. More than 12 pages needs an explicit range."),
                ("Download JPG or zip", "One page → .jpg. Several pages → .zip of JPEGs."),
            ],
            Self::ToText => &[
                ("Drop the PDF", "Born-digital files (exports, invoices with fonts) work."),
                ("Optional range", "Leave blank for every page."),
                ("Copy or download", "The text appears on the page. Download a .txt for your editor."),
            ],
        }
    }

    fn faq(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Compress => &[
                ("Will this hit a kilobyte target like UnderKb?", "No. This is a PDF rewrite, not a photo budget. Huge JPEGs inside the file stay huge."),
                ("Do you store my PDF?", "The result lives in memory about 30 minutes so you can download it, then it expires."),
            ],
            Self::Merge => &[
                ("Is there a page limit?", "The cap is size (20 MB per file, eight files), not page count. Very large page trees can still time out."),
                ("Are forms and outlines preserved?", "Page content and resources are. Interactive structure across files is not rebuilt."),
            ],
            Self::Split => &[
                ("What does 4-z mean?", "From page 4 through the last page. r1 is the last page; r2-r1 is the last two."),
                ("Why a zip?", "Bursting many pages as separate downloads is awkward in the browser. One zip is one click."),
            ],
            Self::FromJpg => &[
                ("Do you keep EXIF?", "The JPEG inside the PDF is a re-encode. Camera metadata is not copied."),
                ("HEIC from iPhone?", "Export JPG first, or use UnderKb, then drop the JPG here."),
            ],
            Self::ToJpg => &[
                ("Is this print quality?", "No. It is a screen-resolution raster for email, CMS, or a quick preview."),
                ("CMYK or spot colors?", "The renderer maps them as best it can. Proofing is not the goal."),
            ],
            Self::ToText => &[
                ("Is this OCR?", "No. OCR is a different product. We only read text that is already in the PDF."),
                ("Why is the order weird?", "We sort runs top-to-bottom, then left-to-right. Multi-column layouts can interleave."),
            ],
        }
    }
}

pub fn public_origin() -> String {
    std::env::var("SITE_URL")
        .ok()
        .map(|s| s.trim().trim_end_matches('/').to_string())
        .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
        .unwrap_or_else(|| "https://pdfforge.fly.dev".into())
}

pub fn canonical_url(path: &str) -> String {
    format!("{}{path}", public_origin())
}

pub fn chrome_store_url() -> Option<String> {
    std::env::var("CHROME_STORE_URL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| s.starts_with("https://"))
}

pub fn seo_footer_links() -> View {
    view! {
        <nav class="seo-links" aria-label="PDFForge tools">
            <NavLink href="/pdf-compress">"Compress"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/pdf-merge">"Merge"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/pdf-split">"Split"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/jpg-to-pdf">"JPG→PDF"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/pdf-to-jpg">"PDF→JPG"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/pdf-to-text">"Text"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/privacy">"Privacy"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/terms">"Terms"</NavLink>
            <span aria-hidden="true">" · "</span>
            <NavLink href="/extension">"Extension"</NavLink>
        </nav>
    }
}

/// Canonical family order. Each app skips itself in the footer and home cards.
const FAMILY: &[(&str, &str, &str)] = &[
    (
        "YouTubeForge",
        "YouTube transcript, MP3, SRT, and translation.",
        "https://youtubetotext.fly.dev",
    ),
    (
        "UnderKb",
        "Compress images to a real KB target. JPG, WebP, PNG.",
        "https://underkb.fly.dev",
    ),
    (
        "PDFForge",
        "Merge, split, compress PDFs. JPG ↔ PDF and extract text.",
        "https://pdfforge.fly.dev",
    ),
    (
        "PlacaQR",
        "3D-printable QR — stand, tile, keychain, or plaque.",
        "https://placaqr.fly.dev",
    ),
    (
        "Linkprobe",
        "Paste a URL. See which links work and which 404.",
        "https://linkprobe.fly.dev",
    ),
    (
        "Svgsport",
        "Paste a page. Download every SVG as a zip.",
        "https://svgsport.fly.dev",
    ),
    (
        "Billloom",
        "Invoice, quote, and receipt PDFs. No account, no watermark.",
        "https://billloom.fly.dev",
    ),
];

const SELF: &str = "PDFForge";

pub fn sister_apps() -> View {
    let cards = FAMILY
        .iter()
        .copied()
        .filter(|(name, _, _)| *name != SELF)
        .map(|(name, blurb, href)| {
            let name = name.to_string();
            let blurb = blurb.to_string();
            let href = href.to_string();
            view! {
                <li>
                    <a href={href} class="tool-card sister-card" rel="noopener">
                        <h3 class="tool-card-title">{name}</h3>
                        <p class="tool-card-blurb">{blurb}</p>
                        <span class="tool-card-cta">"Open"</span>
                    </a>
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="sister-apps" aria-label="Other apps from us">
            <p class="eyebrow">"Also from us"</p>
            <h2>"Free tools, same idea"</h2>
            <p class="hint">
                "No account. Paste, convert, download. Transcripts, images under a KB cap, 3D QR, and invoices."
            </p>
            <ul class="tool-grid">{cards}</ul>
        </nav>
    }
}

pub fn sister_apps_links() -> View {
    let items = FAMILY
        .iter()
        .copied()
        .filter(|(name, _, _)| *name != SELF)
        .enumerate()
        .map(|(i, (name, _, href))| {
            let name = name.to_string();
            let href = href.to_string();
            if i == 0 {
                view! { <a href={href} rel="noopener">{name}</a> }
            } else {
                view! {
                    <span aria-hidden="true">" · "</span>
                    <a href={href} rel="noopener">{name}</a>
                }
            }
        })
        .collect::<Vec<_>>();

    view! {
        <nav class="sister-apps-links" aria-label="Also from us">
            <span>"Also from us:"</span>
            " "
            {items}
        </nav>
    }
}

pub fn hero_particles() -> View {
    view! {
        <div class="hero-particles" data-hero-particles="" aria-hidden="true"></div>
    }
}

pub fn page(tool: Tool, surface: Surface) -> View {
    set_page_title(tool.title());
    set_page_description(tool.description());
    if surface == Surface::Seo {
        set_page_canonical(canonical_url(tool.path()));
    }

    let faq_ld = json!({
        "@context": "https://schema.org",
        "@type": "FAQPage",
        "mainEntity": tool.faq().iter().map(|(q, a)| json!({
            "@type": "Question",
            "name": q,
            "acceptedAnswer": { "@type": "Answer", "text": a }
        })).collect::<Vec<_>>()
    });
    let page_ld = json!({
        "@context": "https://schema.org",
        "@type": "WebPage",
        "name": tool.title(),
        "description": tool.description(),
        "url": canonical_url(tool.path())
    });

    let howto: Vec<View> = tool
        .howto()
        .iter()
        .map(|(h, p)| {
            view! {
                <li>
                    <h3>{*h}</h3>
                    <p>{*p}</p>
                </li>
            }
        })
        .collect();
    let faq: Vec<View> = tool
        .faq()
        .iter()
        .map(|(q, a)| {
            view! {
                <details>
                    <summary>{*q}</summary>
                    <p>{*a}</p>
                </details>
            }
        })
        .collect();
    let more = more_tools(tool);
    let form = tool::form(tool);
    let class = if surface == Surface::App {
        // `/app/*` mirrors the SEO landing: keep it out of the index and point
        // the canonical at the ranking URL so the two never compete.
        set_page_robots("noindex, follow");
        set_page_canonical(canonical_url(tool.path()));
        "home-page app-shell"
    } else {
        // Page-scoped JSON-LD in <head> (with the CSP nonce) replaces the kit's
        // site-wide blocks so a tool page carries exactly one FAQPage.
        set_page_json_ld(json!([page_ld, faq_ld]).to_string());
        "home-page"
    };

    view! {
        <main class={class} lang="en">
            {hero_particles()}
            <section class="hero">
                <p class="eyebrow">{tool.eyebrow()}</p>
                <h1>{tool.h1()}</h1>
                <p class="hero-lead">{tool.lead()}</p>
                {form}
            </section>
            <section class="howto" aria-labelledby="howto-title">
                <h2 id="howto-title">"How to use it"</h2>
                <ol class="howto-grid">{howto}</ol>
            </section>
            {crate::ads::slot("landing-mid", "infeed")}
            <section class="faq" aria-labelledby="faq-title">
                <h2 id="faq-title">"FAQ"</h2>
                <div class="faq-list">{faq}</div>
            </section>
            <section class="related-tools" aria-labelledby="more-title">
                <h2 id="more-title">"Finished? Other PDFForge tools"</h2>
                <ul class="feature-grid">{more}</ul>
            </section>
        </main>
    }
}

fn more_tools(current: Tool) -> Vec<View> {
    Tool::all()
        .into_iter()
        .filter(|t| *t != current)
        .map(|t| {
            let href = t.path();
            let title = t.card_title();
            let blurb = t.card_blurb();
            view! {
                <li>
                    <h3>
                        <NavLink href={href}>{title}</NavLink>
                    </h3>
                    <p>{blurb}</p>
                </li>
            }
        })
        .collect()
}
