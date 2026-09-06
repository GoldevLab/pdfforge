use resuma::prelude::*;

use crate::landing::canonical_url;
use crate::site;

pub fn page(_req: FlowRequest) -> View {
    set_page_title("Privacy | PDFForge");
    set_page_description(
        "How PDFForge handles the PDFs you upload, short-lived downloads, and ads.",
    );
    set_page_canonical(canonical_url("/privacy"));
    let contact = match site::contact_email() {
        Some(email) => {
            let href = format!("mailto:{email}");
            view! {
                <p>
                    "Questions: "
                    <a href={href}>{email}</a>
                    " or "
                    <a href="https://github.com/GoldevLab/pdfforge/issues" rel="noopener">"GitHub"</a>
                    ". "
                    <NavLink href="/terms">"Terms"</NavLink>
                    "."
                </p>
            }
        }
        None => view! {
            <p>
                "Questions: "
                <a href="https://github.com/GoldevLab/pdfforge/issues" rel="noopener">"open an issue on GitHub"</a>
                " (set CONTACT_EMAIL on the server to show a mailbox). "
                <NavLink href="/terms">"Terms"</NavLink>
                "."
            </p>
        },
    };
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
                "Reserved ad slots may show Google AdSense when a publisher id is configured. Google and its partners may set cookies or use similar identifiers to show ads and measure them. See "
                <a href="https://policies.google.com/technologies/ads" rel="noopener">"Google’s advertising policies"</a>
                " and "
                <a href="https://adssettings.google.com/" rel="noopener">"Ad Settings"</a>
                ". Publisher ads.txt is served at "
                <a href="/ads.txt">"/ads.txt"</a>
                " when a publisher id is configured."
            </p>
            <h2>"Browser extension"</h2>
            <p>
                "The optional Chrome extension only opens this website. It does not show Google AdSense inside the popup. Ads stay on these pages."
            </p>
            <h2>"Analytics"</h2>
            <p>
                "If GA4_ID or PLAUSIBLE_DOMAIN is set on the server, page views are measured so we can see which landings work."
            </p>
            <h2>"Contact"</h2>
            {contact}
        </main>
    }
}
