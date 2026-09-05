//! PDFForge — merge, split, compress, PDF→JPG, PDF→text (Resuma Flow).

mod ads;
mod api;
mod family;
mod imgpdf;
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

fn theme_picker() -> View {
    view! {
        <div class="theme-wrap">
            <Popup id="themes" positions="bottom left top right" class="theme-popover">
                <button
                    slot="anchor"
                    type="button"
                    class="theme-btn"
                    aria-haspopup="dialog"
                    aria-label="Change site theme"
                    title="Themes"
                >
                    <span class="theme-btn-wheel" aria-hidden="true"></span>
                    <span class="theme-btn-label">"Theme"</span>
                </button>
                <div class="theme-popover-body" role="dialog" aria-label="Choose a theme">
                    <p class="theme-popover-kicker">"Live themes"</p>
                    <p class="theme-popover-lead">
                        "Press is PDFForge’s ink palette. The rest are official Resuma palettes."
                    </p>
                    <div class="theme-grid" role="group" aria-label="Site palettes">
                        <ThemeSwitch id="press">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#0c1018 0 120deg, #3b82f6 120deg 240deg, #93c5fd 240deg 360deg)"></span>
                                <span>"Press"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="paper">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#eceff4 0 120deg, #2563eb 120deg 240deg, #0f172a 240deg 360deg)"></span>
                                <span>"Paper"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="slate">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#f4efe6 0 120deg, #c2410c 120deg 240deg, #1c1917 240deg 360deg)"></span>
                                <span>"Slate"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="midnight">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#0b1020 0 120deg, #818cf8 120deg 240deg, #e6e8ee 240deg 360deg)"></span>
                                <span>"Midnight"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="ember">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#1a100c 0 120deg, #f59e0b 120deg 240deg, #e8a87c 240deg 360deg)"></span>
                                <span>"Ember"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="aurora">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#0a1628 0 120deg, #22d3ee 120deg 240deg, #a78bfa 240deg 360deg)"></span>
                                <span>"Aurora"</span>
                            </button>
                        </ThemeSwitch>
                        <ThemeSwitch id="forest">
                            <button type="button" class="theme-opt">
                                <span class="theme-swatch" style="background: conic-gradient(#0c1410 0 120deg, #34d399 120deg 240deg, #6ee7b7 240deg 360deg)"></span>
                                <span>"Forest"</span>
                            </button>
                        </ThemeSwitch>
                    </div>
                </div>
            </Popup>
        </div>
    }
}

fn chrome(body: View) -> View {
    view! {
        <div class="app">
            <div class="liquid-orbs" aria-hidden="true">
                <div class="liquid-blob liquid-blob-a"></div>
                <div class="liquid-blob liquid-blob-b"></div>
                <div class="liquid-blob liquid-blob-c"></div>
            </div>
            <header class="site-header">
                <div class="header-inner">
                    <NavLink href="/" class="brand" activeClass="is-active" exact=true>
                        <span class="brand-mark" aria-hidden="true">"PDF"</span>
                        <span class="brand-name">"PDFForge"</span>
                    </NavLink>
                    {theme_picker()}
                    <span class="nav-progress" aria-hidden="true"></span>
                </div>
            </header>
            {body}
            <footer class="site-footer">
                {crate::landing::seo_footer_links()}
                {crate::landing::sister_apps_links()}
                <p>
                    <strong>"PDFForge"</strong>
                    " — merge, split, compress, JPG↔PDF, extract text. No account."
                </p>
            </footer>
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
        </main>
    })
}

fn seo_kit() -> SeoKit {
    let origin = crate::landing::public_origin();
    let mut kit = SeoKit::new("PDFForge", &origin)
        .with_locale("en_US")
        .with_keywords(
            "compress pdf, merge pdf, split pdf, jpg to pdf, pdf to jpg, pdf to text, pdfforge",
        )
        .with_llms_summary(
            "PDFForge merges, splits, and compresses PDFs, converts JPG/PNG to PDF, PDF to JPG, and extracts text. No account. Pure Rust.",
        )
        .with_default_json_ld()
        .push_json_ld(json!({
            "@context": "https://schema.org",
            "@type": "WebApplication",
            "name": "PDFForge",
            "url": crate::landing::public_origin(),
            "applicationCategory": "UtilitiesApplication",
            "operatingSystem": "Web",
            "offers": {"@type": "Offer", "price": "0", "priceCurrency": "USD"},
            "description": "Free PDF merge, split, compress, JPG to PDF, PDF to JPG, and PDF to text. No account."
        }));
    kit.theme_color = Some("#0c1018".into());
    kit.author = "PDFForge".into();
    kit.llms_sections = vec![
        (
            "How to use".into(),
            "POST /api/compress file. /api/merge files. /api/split range,mode. /api/from-jpg images. /api/to-jpg range,dpi. /api/to-text range.".into(),
        ),
        (
            "Pages".into(),
            "/pdf-compress /pdf-merge /pdf-split /jpg-to-pdf /pdf-to-jpg /pdf-to-text. /privacy /terms.".into(),
        ),
    ];
    kit
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // `with_seo_kit` owns keywords/author/theme-color meta, JSON-LD, and the
    // `/robots.txt` + `/llms.txt` routes (AI crawler policy included).
    let head = "<link rel=\"icon\" href=\"/icon.svg\" type=\"image/svg+xml\" /><script type=\"module\" src=\"/js/pdfforge.js?v=5\"></script>";
    const ICON: &[u8] = include_bytes!("icon.svg");
    let public = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("public");

    FlowApp::new()
        .with_title("PDFForge — merge, split, compress, JPG to PDF, PDF to JPG and text")
        .with_description(
            "Free PDF tools: compress, merge, split, JPG to PDF, PDF to JPG, extract text. No account.",
        )
        .with_site_url(crate::landing::public_origin())
        .with_og_image("/icon.svg")
        .with_head(head)
        .with_seo_kit(seo_kit())
        .with_html_theme(
            HtmlTheme::new(["press", "paper", "slate", "midnight", "ember", "aurora", "forest"])
                .dark(["press", "midnight", "ember", "aurora", "forest"])
                .cookie("pdfforge_theme")
                .storage_key("pdfforge-theme"),
        )
        .with_stylesheet("/css/pdfforge.css")
        // `/app/*` are noindex mirrors of the SEO landings; keep them out of the sitemap.
        .with_sitemap_exclude(
            std::iter::once("/app").chain(family::Tool::all().into_iter().map(|t| t.app_path())),
        )
        .static_asset("/icon.svg", ICON, "image/svg+xml")
        .with_public_dir(public)
        .with_pwa(FlowPwaConfig {
            name: "PDFForge".into(),
            short_name: "PDFForge".into(),
            description: "Merge, split, compress PDFs. Convert to JPG or text.".into(),
            theme_color: "#3b82f6".into(),
            background_color: "#0c1018".into(),
            start_url: "/".into(),
            scope: "/".into(),
            cache_version: "pdf-6".into(),
            display: "standalone".into(),
            orientation: "any".into(),
            lang: "en".into(),
            icon_char: Some("P".into()),
            precache_paths: vec![
                "/themes.css".into(),
                "/css/pdfforge.css".into(),
                "/js/pdfforge.js?v=5".into(),
            ],
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
        .route("/api/from-jpg", post(api::from_jpg_upload).options(api::preflight))
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
