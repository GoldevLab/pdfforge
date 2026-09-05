use resuma::prelude::*;

use crate::landing::canonical_url;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Privacy | PDFForge");
    set_page_description(
        "How PDFForge handles the PDFs you upload, short-lived downloads, and ads.",
    );
    set_page_canonical(canonical_url("/privacy"));
    view! {
        <main class="content-section privacy-page">
            <p class="eyebrow">"Legal"</p>
            <h1>"Privacy"</h1>
            <p class="hero-lead">
                "PDFForge is a no-account tool. We do not create user profiles, and we do not keep a library of the files you drop."
            </p>
            <h2>"What we process"</h2>
            <p>
                "When you upload a PDF or image we run the job on this server (compress, merge, split, convert, or extract text) and keep the result in memory about 30 minutes so you can download it. Then it expires."
            </p>
            <h2>"What we do not do"</h2>
            <p>
                "No sign-up, no mailing list, no stored gallery. Use the tools on files you are allowed to process."
            </p>
            <h2>"Advertising"</h2>
            <p>
                "Reserved ad slots may show Google AdSense when a publisher id is configured. See "
                <a href="https://policies.google.com/technologies/ads" rel="noopener">"Google’s advertising policies"</a>
                "."
            </p>
            <h2>"Contact"</h2>
            <p>
                "Questions: "
                <a href="https://github.com/GoldevLab/pdfforge/issues" rel="noopener">"open an issue on GitHub"</a>
                " (set CONTACT_EMAIL on the server to show a mailbox). "
                <NavLink href="/terms">"Terms"</NavLink>
                "."
            </p>
        </main>
    }
}
