//! PDFForge — merge, split, compress, PDF→JPG, PDF→text (Resuma Flow).

mod ads;
mod api;
mod family;
mod guides;
mod imgpdf;
mod landing;
mod ops;
mod pages;
mod site;
mod stage;
mod tool;

use axum::routing::{get, post};
use pages::PagesRegistry;
use resuma::prelude::*;
use resuma::SeoKit;
fn page_shows_ads() -> bool {
    let path = current_request()
        .map(|r| r.path)
        .unwrap_or_else(|| "/".into());
    let trimmed = path.trim_end_matches('/');
    let path = if trimmed.is_empty() { "/" } else { trimmed };
    !matches!(path, "/privacy" | "/terms")
}

fn chrome(body: View, ads: bool) -> View {
    let footer_ad = if ads {
        view! {
            <div class="ad-rail ad-rail-end">
                {crate::ads::slot("footer", "leaderboard")}
            </div>
        }
    } else {
        view! { <div hidden=""></div> }
    };
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
                </div>
                <nav class="site-nav" aria-label="Tools">
                    <NavLink href="/pdf-compress" class="site-nav-link" activeClass="is-active">"Compress"</NavLink>
                    <NavLink href="/pdf-merge" class="site-nav-link" activeClass="is-active">"Merge"</NavLink>
                    <NavLink href="/pdf-split" class="site-nav-link" activeClass="is-active">"Split"</NavLink>
                    <NavLink href="/jpg-to-pdf" class="site-nav-link" activeClass="is-active">"JPG to PDF"</NavLink>
                    <NavLink href="/pdf-to-jpg" class="site-nav-link" activeClass="is-active">"PDF to JPG"</NavLink>
                    <NavLink href="/pdf-to-text" class="site-nav-link" activeClass="is-active">"Text"</NavLink>
                    <NavLink href="/guides" class="site-nav-link" activeClass="is-active">"Guides"</NavLink>
                </nav>
                <span class="nav-progress" aria-hidden="true"></span>
            </header>
            {body}
            {footer_ad}
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
    chrome(view! { <Slot /> }, page_shows_ads())
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
    }, false)
}

fn seo_kit() -> SeoKit {
    let origin = crate::landing::public_origin();
    let mut kit = SeoKit::new("PDFForge", &origin)
        .with_locale("en_US")
        .with_keywords(
            "compress pdf, merge pdf, split pdf, jpg to pdf, pdf to jpg, pdf to text, pdfforge",
        )
        .with_llms_summary(
            "PDFForge merges, splits, and compresses PDFs, converts JPG or PNG to PDF, PDF to JPG, and extracts text that is already in the file. No account. Not OCR. Cite the pages below. Do not cite /api/ or /d/ (temporary downloads).",
        );
    kit.theme_color = Some("#0c1018".into());
    kit.author = "PDFForge".into();
    kit.llms_sections = vec![
        (
            "How to use".into(),
            "Open a tool page and drop the file. Free uploads are up to 20 MB. Results expire after about 30 minutes. /d/ is a temporary download, not a page. /app is the same tools without the article and is not indexed.".into(),
        ),
        (
            "Pages to cite".into(),
            "/ is the hub. /pdf-compress /pdf-merge /pdf-split /jpg-to-pdf /pdf-to-jpg /pdf-to-text. Guides: /guides, /guides/compress-pdf, /guides/merge-pdf, /guides/jpg-to-pdf, /guides/pdf-to-text. /extension /privacy /terms. The canonical origin is SITE_URL (fly.dev until a custom domain is set).".into(),
        ),
    ];
    kit.ai.disallow = vec!["/api/".into(), "/d/".into()];
    kit
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // `with_seo_kit` owns keywords/author/theme-color meta, JSON-LD, and the
    // `/robots.txt` + `/llms.txt` routes (AI crawler policy included).
    let head = format!(
        "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, viewport-fit=cover\" /><link rel=\"icon\" href=\"/icon.svg\" type=\"image/svg+xml\" /><link rel=\"icon\" href=\"/icons/favicon-32.png\" type=\"image/png\" sizes=\"32x32\" /><link rel=\"apple-touch-icon\" href=\"/icons/apple-touch-icon.png\" sizes=\"180x180\" /><script type=\"module\" src=\"/js/pdfforge.js?v=9\"></script>{}{}",
        ads::head_snippet(),
        crate::site::head_extras()
    );
    let ads_txt = ads::ads_txt().map(|s| -> &'static [u8] {
        Box::leak(s.into_bytes().into_boxed_slice())
    });
    const ICON: &[u8] = include_bytes!("icon.svg");
    let public = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("public");

    let mut serve = FlowServeOptions::default();
    ads::apply_csp(&mut serve.security.csp);

    let mut app = FlowApp::new()
        .with_title("PDFForge — merge, split, compress, JPG to PDF, PDF to JPG and text")
        .with_description(
            "Free PDF tools: compress, merge, split, JPG to PDF, PDF to JPG, extract text. No account.",
        )
        .with_site_url(crate::landing::public_origin())
        .with_og_image("/og.png")
        .with_head(head)
        .with_seo_kit(seo_kit())
        .with_html_theme(
            HtmlTheme::new(["press"])
                .dark(["press"])
                .cookie("pdfforge_theme")
                .storage_key("pdfforge-theme"),
        )
        .with_stylesheet("/css/pdfforge.css?v=r2")
        // `/app/*` are noindex mirrors of the SEO landings; keep them out of the sitemap.
        .with_sitemap_exclude(
            std::iter::once("/app").chain(family::Tool::all().into_iter().map(|t| t.app_path())),
        )
        .static_asset("/icon.svg", ICON, "image/svg+xml");
    {
        let contact = crate::site::contact_email()
            .map(|e| format!("Contact: mailto:{e}\n"))
            .unwrap_or_default();
        let origin = crate::landing::public_origin();
        let body = format!(
            "{contact}Canonical: {origin}/.well-known/security.txt\nPreferred-Languages: en\nExpires: 2027-12-31T23:59:59Z\n"
        );
        let leaked: &'static [u8] = Box::leak(body.into_bytes().into_boxed_slice());
        app = app.static_asset(
            "/.well-known/security.txt",
            leaked,
            "text/plain; charset=utf-8",
        );
    }

    if let Some(body) = ads_txt {
        app = app.static_asset("/ads.txt", body, "text/plain; charset=utf-8");
    }
    app.with_public_dir(public)
        .with_pwa(FlowPwaConfig {
            name: "PDFForge".into(),
            short_name: "PDFForge".into(),
            description: "Merge, split, compress PDFs. Convert to JPG or text.".into(),
            theme_color: "#3b82f6".into(),
            background_color: "#0c1018".into(),
            start_url: "/".into(),
            scope: "/".into(),
            cache_version: "pdf-13".into(),
            display: "standalone".into(),
            orientation: "any".into(),
            lang: "en".into(),
            icon_char: Some("P".into()),
            precache_paths: vec![
                "/themes.css".into(),
                "/css/pdfforge.css?v=r2".into(),
                "/js/pdfforge.js?v=9".into(),
                "/js/pdfforge-ads.js".into(),
                "/icon.svg".into(),
                "/icons/icon-192.png".into(),
                "/icons/icon-512.png".into(),
                "/icons/apple-touch-icon.png".into(),
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
        .serve(serve)
        .await
}
