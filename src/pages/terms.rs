use resuma::prelude::*;

use crate::landing::canonical_url;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Terms | PDFForge");
    set_page_description("What PDFForge processes and what you may upload.");
    set_page_canonical(canonical_url("/terms"));
    view! {
        <main class="content-section privacy-page">
            <p class="eyebrow">"Legal"</p>
            <h1>"Terms of use"</h1>
            <p class="hero-lead">
                "PDFForge rewrites, merges, splits, or converts files you upload. We are not a file host or a document archive."
            </p>
            <h2>"Your responsibility"</h2>
            <p>
                "Only upload files you have the right to change. Do not use this app to store illegal material or to flood the machine."
            </p>
            <h2>"Limits"</h2>
            <p>
                "Uploads are capped (about 20 MB). Results expire in about 30 minutes. We do not promise every PDF will shrink, or that extracted text matches a complex layout."
            </p>
            <p>
                <NavLink href="/privacy">"Privacy"</NavLink>
            </p>
        </main>
    }
}
