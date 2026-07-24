use crate::data::varieties::{AMBROSIA_INDEX, HUB_URL, VARIETIES};
use rust_i18n::t;
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
            let cls = if is_current { "variety-pill current-variety" } else { "variety-pill future-variety" };
            html! {
                <div class={cls}>
                    <span class="variety-dot"></span>
                    <span class="variety-name">{ v.name }</span>
                    { if is_current {
                        html! { <><span class="variety-badge">{ v.framework }</span><span class="variety-here">{ t!("series.you_are_here") }</span></> }
                    } else {
                        html! { <span class="variety-soon">{ t!("series.coming_soon") }</span> }
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
                        <span>{ t!("series.label") }</span>
                    </p>
                    <h2 class="chapter-title reveal">{ t!("series.title") }</h2>
                    <p class="chapter-intro reveal">
                        { t!("series.description") }
                        <strong>{ "Yew + Rust + WebAssembly" }</strong>{ "." }
                    </p>
                    <div class="series-pills reveal">{ items }</div>
                    <p class="hub-link reveal">
                        <a href={HUB_URL} target="_blank" rel="noopener">{ t!("series.hub_link") }</a>
                    </p>
                </div>
            </section>
        }
    }
}
