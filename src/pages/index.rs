use resuma::prelude::*;

use crate::family::Tool;

pub fn page(_req: FlowRequest) -> View {
    let cards: Vec<View> = Tool::all().into_iter().map(tool_card).collect();
    view! {
        <main class="home-page home-hub" lang="en">
            <section class="hero">
                <p class="eyebrow">"PDF tools"</p>
                <h1>"What do you want to do with the PDF?"</h1>
                <p class="hero-lead">
                    "Compress, merge, split, turn pages into JPG, or extract text. No account, no watermark. Up to 20 MB."
                </p>
            </section>
            {crate::ads::slot("home-hero", "infeed")}
            <section class="tool-pick" aria-labelledby="tools-title">
                <h2 id="tools-title">"Tools"</h2>
                <div class="tool-grid">{cards}</div>
            </section>
            {crate::ads::slot("home-mid", "infeed")}
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
                        <summary>"Is PDF to text OCR?"</summary>
                        <p>"No. Only selectable text. Scans without a text layer will be empty."</p>
                    </details>
                </div>
            </section>
        </main>
    }
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
