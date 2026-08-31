use resuma::prelude::*;

use crate::family::Tool;

pub fn page(_req: FlowRequest) -> View {
    let cards: Vec<View> = Tool::all()
        .into_iter()
        .map(|t| {
            let href = t.app_path();
            let title = t.card_title();
            let blurb = t.card_blurb();
            view! {
                <NavLink href={href} class="tool-card">
                    <h3 class="tool-card-title">{title}</h3>
                    <p class="tool-card-blurb">{blurb}</p>
                    <span class="tool-card-cta">"Open"</span>
                </NavLink>
            }
        })
        .collect();
    view! {
        <main class="home-page app-shell" lang="en">
            <section class="hero">
                <p class="eyebrow">"App"</p>
                <h1>"PDFForge"</h1>
                <p class="hero-lead">"Tools only — no article. Same APIs as the SEO pages."</p>
            </section>
            <div class="tool-grid">{cards}</div>
        </main>
    }
}
