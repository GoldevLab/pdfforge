//! PDFForge — merge, split, compress, PDF→JPG, PDF→text (Resuma Flow).

mod ads;
mod api;
mod family;
mod landing;
mod ops;
mod pages;
mod stage;
mod tool;

use axum::routing::{get, post};
use pages::PagesRegistry;
use resuma::prelude::*;
use resuma::SeoKit;
use serde_json::json;

fn chrome(body: View) -> View {
    view! {
        <div class="app">
            <header class="site-header">
                <div class="header-inner">
                    <NavLink href="/" class="brand" activeClass="is-active" exact=true>
                        <span class="brand-mark" aria-hidden="true">"PDF"</span>
                        <span class="brand-name">"PDFForge"</span>
                    </NavLink>
                    {tool::tools_nav()}
                    <span class="nav-progress" aria-hidden="true"></span>
                </div>
            </header>
            {body}
            <div class="ad-rail">
                {crate::ads::slot("footer", "leaderboard")}
            </div>
            <footer class="site-footer">
                <p>
                    <strong>"PDFForge"</strong>
                    " — PDF tools, no account. Also: "
                    <a href="https://underkb.fly.dev">"UnderKb"</a>
                    ", "
                    <a href="https://youtubetotext.fly.dev">"YouTubeForge"</a>
                    ", "
                    <a href="https://placaqr.fly.dev">"PlacaQR"</a>
                    "."
                </p>
            </footer>
            <div class="ad-rail ad-rail-end">
                {crate::ads::slot("anchor", "leaderboard")}
            </div>
        </div>
    }
}

#[layout("/")]
fn RootLayout() -> View {
    chrome(view! { <Slot /> })
}

fn not_found() -> View {
    chrome(view! {
        <main class="content-section">
            <h1>"Page not found"</h1>
            <p class="hero-lead">"That path does not exist on PDFForge."</p>
            <p>
                <NavLink href="/" class="btn btn-primary">"Go home"</NavLink>
            </p>
            {crate::ads::slot("notfound-mid", "infeed")}
        </main>
    })
}

fn seo_kit() -> SeoKit {
    let mut kit = SeoKit::new("PDFForge", "https://pdfforge.fly.dev")
        .with_locale("en_US")
        .with_keywords(
            "compress pdf, merge pdf, split pdf, pdf to jpg, pdf to text, pdfforge",
        )
        .with_llms_summary(
            "PDFForge merges, splits, and compresses PDFs, converts pages to JPG, and extracts text. No account. Pure Rust (pdq/hayro).",
        )
        .with_default_json_ld()
        .push_json_ld(json!({
            "@context": "https://schema.org",
            "@type": "WebApplication",
            "name": "PDFForge",
            "url": "https://pdfforge.fly.dev",
            "applicationCategory": "UtilitiesApplication",
            "operatingSystem": "Web",
            "offers": {"@type": "Offer", "price": "0", "priceCurrency": "USD"},
            "description": "Free PDF merge, split, compress, PDF to JPG, and PDF to text. No account."
        }));
    kit.theme_color = Some("#1d4ed8".into());
    kit.author = "PDFForge".into();
    kit.llms_sections = vec![(
        "How to use".into(),
        "POST /api/compress file. /api/merge files. /api/split range,mode. /api/to-jpg range,dpi. /api/to-text range.".into(),
    )];
    kit
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let kit = seo_kit();
    let head = format!(
        "{}<link rel=\"icon\" href=\"/icon.svg\" type=\"image/svg+xml\" /><script type=\"module\" src=\"/js/pdfforge.js?v=4\"></script>",
        kit.head_extras()
    );
    let json_ld = serde_json::to_string(&kit.json_ld_blocks).unwrap_or_else(|_| "[]".into());
    let llms: &'static [u8] = Box::leak(kit.llms_txt().into_bytes().into_boxed_slice());
    const ICON: &[u8] = include_bytes!("icon.svg");
    let public = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("public");

    FlowApp::new()
        .with_title("PDFForge — merge, split, compress, PDF to JPG and text")
        .with_description(
            "Free PDF tools: compress, merge, split, convert to JPG, extract text. No account.",
        )
        .with_site_url("https://pdfforge.fly.dev")
        .with_og_image("/og.svg")
        .with_json_ld(json_ld)
        .with_head(head)
        .with_stylesheet("/css/pdfforge.css")
        .static_asset("/llms.txt", llms, "text/plain; charset=utf-8")
        .static_asset("/icon.svg", ICON, "image/svg+xml")
        .with_public_dir(public)
        .with_pwa(FlowPwaConfig {
            name: "PDFForge".into(),
            short_name: "PDFForge".into(),
            description: "Merge, split, compress PDFs. Convert to JPG or text.".into(),
            theme_color: "#1d4ed8".into(),
            background_color: "#0e1218".into(),
            start_url: "/".into(),
            scope: "/".into(),
            cache_version: "pdf-4".into(),
            display: "standalone".into(),
            orientation: "any".into(),
            lang: "en".into(),
            icon_char: Some("P".into()),
            precache_paths: vec!["/css/pdfforge.css".into(), "/js/pdfforge.js".into()],
            shortcuts: vec![
                PwaShortcut {
                    name: "Home".into(),
                    short_name: "Home".into(),
                    url: "/".into(),
                },
                PwaShortcut {
                    name: "Compress PDF".into(),
                    short_name: "Compress".into(),
                    url: "/pdf-compress".into(),
                },
            ],
            offline_title: "You're offline".into(),
            offline_message: "PDFForge needs a connection to process files. Reconnect and try again."
                .into(),
            manifest_icons: Vec::new(),
        })
        .route("/api/compress", post(api::compress_upload).options(api::preflight))
        .route("/api/merge", post(api::merge_upload).options(api::preflight))
        .route("/api/split", post(api::split_upload).options(api::preflight))
        .route("/api/to-jpg", post(api::jpg_upload).options(api::preflight))
        .route("/api/to-text", post(api::text_upload).options(api::preflight))
        .route("/d/{id}", get(api::download))
        .not_found(not_found)
        .auto_pages(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages"),
            PagesRegistry,
        )
        .serve(FlowServeOptions::default())
        .await
}
