//! Reserved AdSense slots — empty frames until units are wired.

use resuma::prelude::*;

pub fn slot(placement: &'static str, size: &'static str) -> View {
    let class = format!("ad-slot ad-slot-{size}");
    view! {
        <aside class={class} data-ad={placement} aria-label="Advertisement">
            <div class="ad-slot-frame">
                <span class="ad-slot-label">"Ad"</span>
            </div>
        </aside>
    }
}
