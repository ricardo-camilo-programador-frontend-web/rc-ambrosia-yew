use crate::data::ambrosia;
use yew::prelude::*;

pub struct Parentage;

impl Component for Parentage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let pollinators = ambrosia::POLLINATORS
            .iter()
            .map(|p| {
                html! { <span class="region-tag">{ p }</span> }
            })
            .collect::<Html>();

        html! {
            <section class="chapter" id="parentage">
                <div class="chapter-wide">
                    <p class="chapter-subtitle reveal">{ "Bloodline" }</p>
                    <h2 class="chapter-title reveal">{ "A Noble Parentage" }</h2>
                    <p class="reveal" style="text-align: center; margin-bottom: 2rem;">
                        { "Ambrosia was a chance seedling, but it grew in an orchard of giants. Genetic analysis suggests it is a natural cross between two legendary varieties:" }
                    </p>
                    <div class="parentage-diagram reveal-scale">
                        <div class="parent-row">
                            <div class="parent-node">
                                <strong>{ ambrosia::PARENTAGE_FATHER }</strong>
                                <p style="font-size: 0.875rem; color: var(--ambrosia-sage);">{ "Suspected Father" }</p>
                            </div>
                            <div class="parent-node">
                                <strong>{ ambrosia::PARENTAGE_MOTHER }</strong>
                                <p style="font-size: 0.875rem; color: var(--ambrosia-sage);">{ "Suspected Mother" }</p>
                            </div>
                        </div>
                        <div class="parent-connector"></div>
                        <div class="parent-node child">
                            <strong>{ "🍎 Ambrosia" }</strong>
                            <p style="font-size: 0.875rem; color: var(--ambrosia-red);">{ "The Result" }</p>
                        </div>
                    </div>
                    <p class="reveal" style="text-align: center; margin-top: 2rem;">
                        { "Pollinating partners: " }
                    </p>
                    <div class="region-list reveal">{ pollinators }</div>
                </div>
            </section>
        }
    }
}
