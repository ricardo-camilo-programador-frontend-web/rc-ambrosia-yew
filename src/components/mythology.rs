use crate::data::ambrosia;
use yew::prelude::*;

pub struct Mythology;

impl Component for Mythology {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="chapter mythology-section dark-section" id="mythology">
                <div class="mythology-overlay"></div>
                <div class="chapter-grid-two mythology-content">
                    <div class="mythology-text reveal-left">
                        <p class="section-label section-label-light">
                            <span class="section-number">{"05"}</span>
                            <span class="section-divider"></span>
                            <span>{"MYTHOLOGY"}</span>
                        </p>
                        <h2 class="chapter-title chapter-title-light">{ "Food of the Gods" }</h2>
                        <p class="mythology-body">
                            { "In ancient Greek mythology, ambrosia was the food of the Olympians, associated with longevity and immortality. When Wilfrid Mennell tasted the apple that had mysteriously appeared in his orchard, its honeyed sweetness evoked this ancient myth." }
                        </p>
                        <blockquote class="mythology-quote">
                            { ambrosia::MYTHOLOGY_QUOTE }
                        </blockquote>
                        <div class="mythology-seal">
                            { "ANCIENT MYTH \u{2022} MODERN AGRICULTURE" }
                        </div>
                    </div>
                    <div class="mythology-visual reveal-scale">
                        <div class="statue-placeholder">
                            <span>{ "\u{1F3DB}\u{FE0F}" }</span>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
