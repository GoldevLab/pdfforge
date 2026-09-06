use resuma::prelude::*;

use crate::landing::canonical_url;
use crate::landing::chrome_store_url;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Chrome extension | PDFForge");
    set_page_description(
        "Open PDFForge from the toolbar: compress, merge, split, JPG to PDF, PDF to JPG, extract text.",
    );
    set_page_canonical(canonical_url("/extension"));
    let store_block = match chrome_store_url() {
        Some(url) => view! {
            <p>
                <a class="btn btn-primary" href={url} rel="noopener">"Add from Chrome Web Store"</a>
            </p>
        },
        None => view! {
            <p class="hint">
                "The Web Store listing is not public yet. In chrome://extensions turn on Developer mode and Load unpacked from the GitHub folder extension/."
            </p>
            <p>
                <a class="btn btn-primary" href="https://github.com/GoldevLab/pdfforge/tree/main/extension" rel="noopener">
                    "extension/ on GitHub"
                </a>
            </p>
        },
    };
    view! {
        <main class="content-section privacy-page">
            <p class="eyebrow">"Browser"</p>
            <h1>"PDFForge extension"</h1>
            <p class="hero-lead">
                "Toolbar shortcuts to every PDFForge tool. Right-click a .pdf link to open Compress. The file still uploads on the website — that is where ads run, which Google AdSense allows."
            </p>
            {store_block}
            <h2>"What it does"</h2>
            <ol class="howto-grid">
                <li>
                    <h3>"Toolbar"</h3>
                    <p>"Compress, merge, split, JPG↔PDF, and text — each button opens the matching page."</p>
                </li>
                <li>
                    <h3>"Right-click"</h3>
                    <p>"On a link that ends in .pdf, Open PDF in PDFForge."</p>
                </li>
            </ol>
            <p>
                <NavLink href="/" class="btn btn-ghost">"Use the website"</NavLink>
            </p>
        </main>
    }
}
