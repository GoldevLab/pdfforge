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
            Self::ToJpg => "Raster pages",
            Self::ToText => "Plain text",
        }
    }

    fn h1(self) -> &'static str {
        match self {
            Self::Compress => "Compress a PDF",
            Self::Merge => "Merge PDF files",
            Self::Split => "Split a PDF",
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

pub fn page(tool: Tool, surface: Surface) -> View {
    set_page_title(tool.title());
    set_page_description(tool.description());

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
        "url": format!("https://pdfforge.fly.dev{}", tool.path())
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
        "home-page app-shell"
    } else {
        "home-page"
    };
    let seo_block = if surface == Surface::Seo {
        view! {
            {View::raw(format!(
                r#"<script type="application/ld+json">{}</script><script type="application/ld+json">{}</script>"#,
                page_ld, faq_ld
            ))}
        }
    } else {
        view! { <span class="sr-only" hidden=""></span> }
    };

    view! {
        <main class={class} lang="en">
            {seo_block}
            <section class="hero">
                <p class="eyebrow">{tool.eyebrow()}</p>
                <h1>{tool.h1()}</h1>
                <p class="hero-lead">{tool.lead()}</p>
                {form}
            </section>
            {crate::ads::slot("landing-hero", "infeed")}
            <section class="howto" aria-labelledby="howto-title">
                <h2 id="howto-title">"How to use it"</h2>
                <ol class="howto-grid">{howto}</ol>
            </section>
            {crate::ads::slot("landing-mid", "infeed")}
            <section class="faq" aria-labelledby="faq-title">
                <h2 id="faq-title">"FAQ"</h2>
                <div class="faq-list">{faq}</div>
            </section>
            {crate::ads::slot("landing-faq", "infeed")}
            <section class="features" aria-labelledby="more-title">
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
