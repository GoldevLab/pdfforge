//! Dropzone forms for each PDF job.

use resuma::prelude::*;

use crate::family::Tool;

fn dropzone(title: &'static str, hint: &'static str, accept: &'static str, multiple: bool) -> View {
    if multiple {
        view! {
            <label class="drop" data-drop="">
                <input
                    class="drop-input"
                    data-file=""
                    name="file"
                    type="file"
                    accept={accept}
                    multiple=""
                />
                <span class="drop-title">{title}</span>
                <span class="drop-hint" data-filename="">{hint}</span>
            </label>
            <ul class="file-list" data-file-list="" hidden=""></ul>
        }
    } else {
        view! {
            <label class="drop" data-drop="">
                <input
                    class="drop-input"
                    data-file=""
                    name="file"
                    type="file"
                    accept={accept}
                />
                <span class="drop-title">{title}</span>
                <span class="drop-hint" data-filename="">{hint}</span>
            </label>
            <ul class="file-list" data-file-list="" hidden=""></ul>
        }
    }
}

fn result_block(download: &'static str) -> View {
    view! {
        <div class="result" data-result="" hidden="">
            <p class="result-name" data-out-name=""></p>
            <p class="result-stats" data-stats=""></p>
            <pre class="result-text" data-text="" hidden=""></pre>
            <img data-preview="" alt="Preview" hidden="" />
            <a class="btn btn-primary" data-download="" data-r-full="" download="">{download}</a>
        </div>
    }
}

pub fn form(tool: Tool) -> View {
    match tool {
        Tool::Compress => compress(),
        Tool::Merge => merge(),
        Tool::Split => split(),
        Tool::FromJpg => from_jpg(),
        Tool::ToJpg => to_jpg(),
        Tool::ToText => to_text(),
    }
}

fn compress() -> View {
    view! {
        <div id="pdf-app" data-tool="compress" data-endpoint="/api/compress" data-idle="Compress" data-busy="Rewriting…" data-empty="Drop or choose a PDF first.">
            <form class="hero-form" data-form="" method="post" action="/api/compress" enctype="multipart/form-data">
                {dropzone("Drop a PDF, or click to choose", "PDF · max 20 MB", "application/pdf,.pdf", false)}
                <p class="hint">"We rewrite the file and drop unused objects. Scanned photos inside the PDF stay as they are."</p>
                <p class="hint form-error" data-error="" hidden="" role="alert" aria-live="polite"></p>
                <button type="submit" class="btn btn-primary btn-wide" data-submit="">"Compress"</button>
            </form>
            {result_block("Download PDF")}
        </div>
    }
}

fn merge() -> View {
    view! {
        <div id="pdf-app" data-tool="merge" data-endpoint="/api/merge" data-idle="Merge" data-busy="Merging…" data-empty="Drop at least two PDFs.">
            <form class="hero-form" data-form="" method="post" action="/api/merge" enctype="multipart/form-data">
                {dropzone("Drop two or more PDFs", "Up to 8 files · 20 MB each", "application/pdf,.pdf", true)}
                <p class="hint">"Order follows the order you selected in the file picker."</p>
                <p class="hint form-error" data-error="" hidden="" role="alert" aria-live="polite"></p>
                <button type="submit" class="btn btn-primary btn-wide" data-submit="">"Merge"</button>
            </form>
            {result_block("Download merged PDF")}
        </div>
    }
}

fn from_jpg() -> View {
    view! {
        <div id="pdf-app" data-tool="from-jpg" data-endpoint="/api/from-jpg" data-idle="Convert" data-busy="Building PDF…" data-empty="Drop JPG or PNG files first.">
            <form class="hero-form" data-form="" method="post" action="/api/from-jpg" enctype="multipart/form-data">
                {dropzone("Drop JPG or PNG files", "Up to 12 images · 20 MB each", "image/jpeg,image/png,.jpg,.jpeg,.png", true)}
                <p class="hint">"Picker order is page order. Each image is one A4 page."</p>
                <p class="hint form-error" data-error="" hidden="" role="alert" aria-live="polite"></p>
                <button type="submit" class="btn btn-primary btn-wide" data-submit="">"Convert to PDF"</button>
            </form>
            {result_block("Download PDF")}
        </div>
    }
}

fn split() -> View {
    view! {
        <div id="pdf-app" data-tool="split" data-endpoint="/api/split" data-idle="Split" data-busy="Splitting…" data-empty="Drop or choose a PDF first.">
            <form class="hero-form" data-form="" method="post" action="/api/split" enctype="multipart/form-data">
                {dropzone("Drop a PDF to split", "PDF · max 20 MB", "application/pdf,.pdf", false)}
                <div class="controls">
                    <label>
                        "Page range"
                        <input data-range="" name="range" type="text" value="1" placeholder="1-3 or 1,4,z" />
                    </label>
                    <label>
                        "Mode"
                        <select data-mode="" name="mode">
                            <option value="range" selected=true>"Keep this range"</option>
                            <option value="each">"Every page (zip)"</option>
                        </select>
                    </label>
                </div>
                <p class="hint">"Ranges use pdq syntax: 1-3, 4-z (through the last page), r1 (last page)."</p>
                <p class="hint form-error" data-error="" hidden="" role="alert" aria-live="polite"></p>
                <button type="submit" class="btn btn-primary btn-wide" data-submit="">"Split"</button>
            </form>
            {result_block("Download")}
        </div>
    }
}

fn to_jpg() -> View {
    view! {
        <div id="pdf-app" data-tool="jpg" data-endpoint="/api/to-jpg" data-idle="Convert" data-busy="Rendering…" data-empty="Drop or choose a PDF first.">
            <form class="hero-form" data-form="" method="post" action="/api/to-jpg" enctype="multipart/form-data">
                {dropzone("Drop a PDF to turn into JPG", "One JPG, or a zip if several pages", "application/pdf,.pdf", false)}
                <div class="controls">
                    <label>
                        "Pages"
                        <input data-range="" name="range" type="text" placeholder="1 or 1-3" />
                    </label>
                    <label>
                        "DPI"
                        <input data-dpi="" name="dpi" type="number" min="72" max="150" value="110" />
                    </label>
                </div>
                <p class="hint">"Leave pages empty to render the first 12. Scans and complex art may look soft — this is a CPU renderer, not Acrobat."</p>
                <p class="hint form-error" data-error="" hidden="" role="alert" aria-live="polite"></p>
                <button type="submit" class="btn btn-primary btn-wide" data-submit="">"Convert to JPG"</button>
            </form>
            {result_block("Download")}
        </div>
    }
}

fn to_text() -> View {
    view! {
        <div id="pdf-app" data-tool="text" data-endpoint="/api/to-text" data-idle="Extract" data-busy="Reading…" data-empty="Drop or choose a PDF first.">
            <form class="hero-form" data-form="" method="post" action="/api/to-text" enctype="multipart/form-data">
                {dropzone("Drop a PDF to extract text", "Selectable text only — not OCR", "application/pdf,.pdf", false)}
                <div class="controls">
                    <label>
                        "Pages (optional)"
                        <input data-range="" name="range" type="text" placeholder="all pages" />
                    </label>
                </div>
                <p class="hint">"Scanned pages with no text layer will come back empty. That is not OCR."</p>
                <p class="hint form-error" data-error="" hidden="" role="alert" aria-live="polite"></p>
                <button type="submit" class="btn btn-primary btn-wide" data-submit="">"Extract text"</button>
            </form>
            {result_block("Download .txt")}
        </div>
    }
}

#[allow(dead_code)]
pub fn tools_nav() -> View {
    let path = current_request()
        .map(|r| r.path)
        .unwrap_or_else(|| "/".into());
    let links: Vec<View> = Tool::all()
        .into_iter()
        .map(|t| {
            let href = t.path();
            let label = t.nav_label();
            let on_app = path == t.app_path();
            let class = if on_app {
                "tool-nav-link is-active"
            } else {
                "tool-nav-link"
            };
            view! {
                <NavLink href={href} class={class} activeClass="is-active">{label}</NavLink>
            }
        })
        .collect();
    let app_on = path == "/app";
    let app_class = if app_on {
        "tool-nav-link is-active"
    } else {
        "tool-nav-link"
    };
    view! {
        <nav class="tool-nav" aria-label="PDF tools">
            <NavLink href="/" class="tool-nav-link" activeClass="is-active" exact=true>"Home"</NavLink>
            {links}
            <NavLink href="/app" class={app_class} activeClass="is-active" exact=true>"App"</NavLink>
        </nav>
    }
}
