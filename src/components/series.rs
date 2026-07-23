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
            let item_class = if is_current {
                "series-item current"
            } else {
                "series-item future"
            };

            html! {
                <div class={item_class}>
                    <div class="series-emoji">{ v.emoji }</div>
                    <div class="series-name">{ v.name }</div>
                    { if is_current {
                        html! { <><div class="series-badge">{ v.framework }</div><div style="font-size: 0.625rem; color: var(--ambrosia-sage);">{ "YOU ARE HERE" }</div></> }
                    } else {
                        html! { <div style="font-size: 0.625rem; color: var(--ambrosia-sage);">{ format!("from {}", v.origin) }</div> }
                    }}
                </div>
            }
        }).collect::<Html>();

        html! {
            <section class="chapter" id="series">
                <div class="chapter-wide">
                    <p class="chapter-subtitle reveal">{ "One Apple, One Framework" }</p>
                    <h2 class="chapter-title reveal">{ "The Series" }</h2>
                    <p class="reveal" style="text-align: center; margin-bottom: 2rem;">
                        { "This article is part of a series where each apple variety is built with a different framework. Ambrosia is built with " }
                        <strong>{ "Yew + Rust + WebAssembly" }</strong>
                        { ". The next variety will use a completely different technology." }
                    </p>
                    <div class="series-grid">
                        { items }
                    </div>
                    <p class="reveal" style="text-align: center; margin-top: 2rem;">
                        <a href={HUB_URL} target="_blank" rel="noopener">
                            { "← View all varieties on the hub" }
                        </a>
                    </p>
                </div>
            </section>
        }
    }
}
