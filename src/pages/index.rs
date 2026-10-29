use resuma::prelude::*;
use serde_json::json;

use crate::family::Tool;
use crate::landing::canonical_url;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("PDFForge — merge, split, compress, JPG to PDF, PDF to JPG and text");
    set_page_description(
        "Free PDF tools: compress, merge, split, JPG to PDF, PDF to JPG, extract text. No account.",
    );
    set_page_canonical(canonical_url("/"));
    set_page_json_ld(home_json_ld());
    let cards: Vec<View> = Tool::all().into_iter().map(tool_card).collect();
    view! {
        <main class="home-page home-hub" lang="en">
            {crate::landing::hero_particles()}
            <section class="hero">
                <p class="eyebrow">"PDF tools"</p>
                <h1>"What do you want to do with the PDF?"</h1>
                <p class="hero-lead">
                    "Compress, merge, split, JPG to PDF, PDF to JPG, or extract text. No account, no watermark. Up to 20 MB."
                </p>
            </section>
            <section class="tool-pick" aria-labelledby="tools-title">
                <h2 id="tools-title">"Tools"</h2>
                <div class="tool-grid">{cards}</div>
            </section>
            <section class="tool-pick" aria-labelledby="guides-title">
                <h2 id="guides-title">"Guides"</h2>
                {crate::guides::home_strip()}
            </section>
            <section class="howto" aria-labelledby="howto-title">
                <h2 id="howto-title">"How it works"</h2>
                <ol class="howto-grid">
                    <li>
                        <h3>"Pick a job"</h3>
                        <p>"SEO pages explain the job. /app is the same tools without the article."</p>
                    </li>
                    <li>
                        <h3>"Drop the PDF"</h3>
                        <p>"Processing is pure Rust (pdq / hayro) on the server. Files expire in about 30 minutes."</p>
                    </li>
                    <li>
                        <h3>"Then another tool"</h3>
                        <p>"After a merge, compress. After a split, JPG or text. That is how the family is meant to be used."</p>
                    </li>
                </ol>
            </section>
            {crate::ads::slot("home-faq", "infeed")}
            <section class="faq" aria-labelledby="faq-title">
                <h2 id="faq-title">"FAQ"</h2>
                <div class="faq-list">
                    <details>
                        <summary>"Do I need an account?"</summary>
                        <p>"No. Open a tool, upload, download."</p>
                    </details>
                    <details>
                        <summary>"Is compress a 200 KB target?"</summary>
                        <p>"No. That is UnderKb for photos. PDF compress here rewrites the file and drops unused objects."</p>
                    </details>
                    <details>
                        <summary>"Can I turn photos into a PDF?"</summary>
                        <p>"Yes. Open JPG to PDF. PNG works too. Each image is one A4 page."</p>
                    </details>
                    <details>
                        <summary>"Is PDF to text OCR?"</summary>
                        <p>"No. Only selectable text. Scans without a text layer will be empty."</p>
                    </details>
                </div>
            </section>
            {crate::landing::sister_apps()}
        </main>
    }
}

fn home_json_ld() -> String {
    let origin = canonical_url("");
    json!({
        "@context": "https://schema.org",
        "@graph": [
            {
                "@type": "Organization",
                "name": "PDFForge",
                "url": origin
            },
            {
                "@type": "WebSite",
                "name": "PDFForge",
                "url": origin
            },
            {
                "@type": "WebApplication",
                "name": "PDFForge",
                "url": origin,
                "applicationCategory": "UtilitiesApplication",
                "operatingSystem": "Web",
                "offers": {"@type": "Offer", "price": "0", "priceCurrency": "USD"},
                "featureList": [
                    "Compress a PDF",
                    "Merge PDFs",
                    "Split a PDF",
                    "Convert JPG to PDF",
                    "Convert PDF pages to JPG",
                    "Extract text already in a PDF"
                ],
                "description": "Free PDF tools: compress, merge, split, JPG to PDF, PDF to JPG, extract text. No account. Not OCR."
            }
        ]
    })
    .to_string()
}

fn tool_card(tool: Tool) -> View {
    let href = tool.path();
    let mark = tool.card_mark();
    let title = tool.card_title();
    let blurb = tool.card_blurb();
    let cta = tool.card_cta();
    view! {
        <NavLink href={href} class="tool-card">
            <span class="tool-card-mark" aria-hidden="true">{mark}</span>
            <h3 class="tool-card-title">{title}</h3>
            <p class="tool-card-blurb">{blurb}</p>
            <span class="tool-card-cta">{cta}</span>
        </NavLink>
    }
}
