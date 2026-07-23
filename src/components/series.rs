use crate::data::varieties::{AMBROSIA_INDEX, HUB_URL, VARIETIES};
use yew::prelude::*;

pub struct Series;

impl Component for Series {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let items = VARIETIES.iter().enumerate().map(|(i, v)| {
            let is_current = i == AMBROSIA_INDEX;
            let pill_class = if is_current { "variety-pill current-variety" } else { "variety-pill future-variety" };
            html! {
                <div class={pill_class}>
                    <span class="variety-dot"></span>
                    <span class="variety-name">{ v.name }</span>
                    { if is_current {
                        html! { <><span class="variety-badge">{ v.framework }</span><span class="variety-here">{ "YOU ARE HERE" }</span></> }
                    } else {
                        html! { <span class="variety-soon">{ "Coming soon" }</span> }
                    }}
                </div>
            }
        }).collect::<Html>();

        html! {
            <section class="chapter light-section" id="series">
                <div class="chapter-wide">
                    <p class="section-label">
                        <span class="section-number">{"11"}</span>
                        <span class="section-divider"></span>
                        <span>{"THE SERIES"}</span>
                    </p>
                    <h2 class="chapter-title reveal">{ "One Apple, One Framework" }</h2>
                    <p class="chapter-intro reveal">
                        { "Each apple variety is built with a different framework. Ambrosia uses " }
                        <strong>{ "Yew + Rust + WebAssembly" }</strong>
                        { "." }
                    </p>
                    <div class="series-pills reveal">{ items }</div>
                    <p class="hub-link reveal">
                        <a href={HUB_URL} target="_blank" rel="noopener">
                            { "\u{2190} View all varieties on the hub" }
                        </a>
                    </p>
                </div>
            </section>
        }
    }
}
