//! Crawlable guides that answer a search and end on the PDF tool.

use resuma::prelude::*;
use serde_json::json;

use crate::family::Tool;
use crate::landing::canonical_url;

struct Guide {
    path: &'static str,
    title: &'static str,
    description: &'static str,
    h1: &'static str,
    lead: &'static str,
    tool: Tool,
    sections: &'static [(&'static str, &'static str)],
    faq: &'static [(&'static str, &'static str)],
    links: &'static [(&'static str, &'static str)],
}

const ALL: &[Guide] = &[COMPRESS, MERGE, FROM_JPG, TO_TEXT];

const COMPRESS: Guide = Guide {
    path: "/guides/compress-pdf",
    title: "Compress a PDF | PDFForge",
    description: "How to make a PDF smaller without an account. PDFForge rewrites the file and drops unused objects. Not a photo-quality slider.",
    h1: "Compress a PDF",
    lead: "Drop the file. The result is a rewritten PDF with unused objects removed. It is not a kilobyte photo compressor, and it does not promise a target size.",
    tool: Tool::Compress,
    sections: &[
        (
            "What actually gets smaller",
            "A document rewrite drops objects nothing on the page still uses. A scan that is already a tight JPEG often barely moves. A export full of unused fonts and images can drop a lot.",
        ),
        (
            "What this is not",
            "There is no OCR and no 'make it 200 KB' control. If you need the pages as pictures, open PDF to JPG after this.",
        ),
        (
            "Limits",
            "Free uploads are up to 20 MB. The result stays available about 30 minutes, then it expires. We do not keep a library of your files.",
        ),
    ],
    faq: &[
        (
            "Will every PDF shrink?",
            "No. If the file is already compact, the rewrite has little to remove.",
        ),
        (
            "Do I need an account?",
            "No.",
        ),
    ],
    links: &[
        ("/pdf-compress", "Open compress"),
        ("/guides/merge-pdf", "Merge first, then compress"),
        ("/pdf-to-jpg", "Export pages as JPG"),
    ],
};

const MERGE: Guide = Guide {
    path: "/guides/merge-pdf",
    title: "Merge PDFs | PDFForge",
    description: "Combine PDF files into one, in the order you drop them. No account.",
    h1: "Merge PDFs into one file",
    lead: "Drop the files in the order you want them. The download is one PDF. Compress it afterward if the host rejects the size.",
    tool: Tool::Merge,
    sections: &[
        (
            "Order is the drop order",
            "The first file is the start of the new PDF. Reorder by dropping again if a page landed in the wrong place.",
        ),
        (
            "Then shrink it",
            "A merge adds bytes. Open Compress PDF with the result when email or a form has a cap.",
        ),
        (
            "What we keep",
            "The merged file expires after about 30 minutes. There is no account.",
        ),
    ],
    faq: &[
        (
            "Can I merge a JPG into a PDF?",
            "Use JPG to PDF for pictures. Merge is for PDFs.",
        ),
        (
            "Is there a page limit?",
            "The limit is the upload size, 20 MB, not a page count.",
        ),
    ],
    links: &[
        ("/pdf-merge", "Open merge"),
        ("/guides/compress-pdf", "Compress the result"),
        ("/guides/jpg-to-pdf", "Turn images into a PDF first"),
    ],
};

const FROM_JPG: Guide = Guide {
    path: "/guides/jpg-to-pdf",
    title: "Convert JPG to PDF | PDFForge",
    description: "Put JPG or PNG images into one PDF. No account.",
    h1: "Convert JPG images to a PDF",
    lead: "Drop one or more images. Each image becomes a page. Download the PDF. This does not OCR the picture into selectable text.",
    tool: Tool::FromJpg,
    sections: &[
        (
            "Pictures, not scans of text",
            "The PDF holds the image. If you need the words as text, that only works when the source is already a PDF with real text. Use PDF to text for that.",
        ),
        (
            "Order",
            "Pages follow the order you add the files.",
        ),
        (
            "Size",
            "A stack of camera photos makes a heavy PDF. Compress it after if a form complains.",
        ),
    ],
    faq: &[
        (
            "Does PNG work?",
            "Yes. JPG and PNG.",
        ),
        (
            "Will the text in the photo be selectable?",
            "No. The page is the picture.",
        ),
    ],
    links: &[
        ("/jpg-to-pdf", "Open JPG to PDF"),
        ("/guides/compress-pdf", "Compress the PDF"),
        ("/pdf-to-text", "Extract text from a real PDF"),
    ],
};

const TO_TEXT: Guide = Guide {
    path: "/guides/pdf-to-text",
    title: "Extract Text from a PDF | PDFForge",
    description: "Copy the text already stored in a PDF. This is not OCR. No account.",
    h1: "Extract text from a PDF",
    lead: "If the PDF was exported from a writer, the words are in the file and this page reads them. A photo of a page has no text layer, so the result is empty. That is not a failure of the reader.",
    tool: Tool::ToText,
    sections: &[
        (
            "Not OCR",
            "We do not look at pixels and guess letters. Scans need a different product.",
        ),
        (
            "Column order",
            "Runs are sorted top to bottom, then left to right. A two-column layout can interleave.",
        ),
        (
            "What you download",
            "A text file of those runs. It expires with the other results, about 30 minutes.",
        ),
    ],
    faq: &[
        (
            "Why is the file empty?",
            "The PDF is probably a scan. There is no text layer to read.",
        ),
        (
            "Can I extract one page range?",
            "Yes. Set the range on the tool.",
        ),
    ],
    links: &[
        ("/pdf-to-text", "Open PDF to text"),
        ("/guides/compress-pdf", "Compress a PDF"),
        ("/pdf-to-jpg", "Export pages as images"),
    ],
};

pub fn render(path: &str) -> View {
    let guide = ALL.iter().find(|g| g.path == path).unwrap_or(&COMPRESS);
    set_page_title(guide.title);
    set_page_description(guide.description);
    set_page_canonical(canonical_url(guide.path));
    set_page_json_ld(guide_json_ld(guide));
    let sections: Vec<View> = guide
        .sections
        .iter()
        .map(|(h, p)| {
            view! {
                <section class="guide-section">
                    <h2>{*h}</h2>
                    <p>{*p}</p>
                </section>
            }
        })
        .collect();
    let faq: Vec<View> = guide
        .faq
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
    let related: Vec<View> = guide
        .links
        .iter()
        .map(|(href, label)| {
            let href = (*href).to_string();
            view! {
                <li>
                    <NavLink href={href}>{*label}</NavLink>
                </li>
            }
        })
        .collect();
    let form = crate::tool::form(guide.tool);
    view! {
        <main class="home-page guide-page" lang="en">
            <section class="hero">
                <p class="eyebrow">"Guide"</p>
                <h1>{guide.h1}</h1>
                <p class="hero-lead">{guide.lead}</p>
                {form}
            </section>
            <article class="content-section guide-article">
                {sections}
                <h2>"Related"</h2>
                <ul class="guide-links">{related}</ul>
                <p class="guide-actions">
                    <NavLink href="/guides" class="btn btn-ghost">"All guides"</NavLink>
                </p>
            </article>
            <section class="faq" aria-labelledby="guide-faq">
                <h2 id="guide-faq">"FAQ"</h2>
                <div class="faq-list">{faq}</div>
            </section>
        </main>
    }
}

pub fn index_page() -> View {
    set_page_title("PDF guides — compress, merge, JPG to PDF, extract text | PDFForge");
    set_page_description(
        "How to compress a PDF, merge files, turn JPG into a PDF, and extract text that is already in the file. Each guide includes the tool.",
    );
    set_page_canonical(canonical_url("/guides"));
    set_page_json_ld(index_json_ld());
    view! {
        <main class="content-section guide-page">
            <p class="eyebrow">"Learn"</p>
            <h1>"PDFForge guides"</h1>
            <p class="hero-lead">"Short pages for the jobs people search for. Each one includes the tool."</p>
            {home_strip()}
        </main>
    }
}

pub fn home_strip() -> View {
    let cards: Vec<View> = ALL
        .iter()
        .map(|g| {
            let href = g.path.to_string();
            view! {
                <li>
                    <a href={href} class="related-card" data-r-nav="true">
                        <strong>{g.h1}</strong>
                        <span>{g.description}</span>
                    </a>
                </li>
            }
        })
        .collect();
    view! { <ul class="related-grid">{cards}</ul> }
}

fn index_json_ld() -> String {
    let origin = crate::landing::public_origin();
    let items: Vec<_> = ALL
        .iter()
        .enumerate()
        .map(|(i, g)| {
            json!({
                "@type": "ListItem",
                "position": i + 1,
                "name": g.h1,
                "url": format!("{origin}{}", g.path)
            })
        })
        .collect();
    json!({
        "@context": "https://schema.org",
        "@type": "ItemList",
        "name": "PDFForge guides",
        "itemListElement": items
    })
    .to_string()
}

fn guide_json_ld(g: &Guide) -> String {
    let origin = crate::landing::public_origin();
    let faq: Vec<_> = g
        .faq
        .iter()
        .map(|(q, a)| {
            json!({
                "@type": "Question",
                "name": q,
                "acceptedAnswer": {"@type": "Answer", "text": a}
            })
        })
        .collect();
    json!({
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "Article",
                "headline": g.h1,
                "description": g.description,
                "mainEntityOfPage": format!("{origin}{}", g.path),
                "author": {"@type": "Organization", "name": "PDFForge"},
                "publisher": {"@type": "Organization", "name": "PDFForge", "url": origin}
            },
            {
                "@type": "FAQPage",
                "mainEntity": faq
            }
        ]
    })
    .to_string()
}
