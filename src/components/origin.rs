use crate::data::ambrosia;
use yew::prelude::*;

#[allow(clippy::module_inception)]
pub struct Origin;

impl Component for Origin {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="chapter light-section" id="origin">
                <div class="chapter-grid-two">
                    <div class="chapter-text-col reveal-left">
                        <p class="section-label">
                            <span class="section-number">{"02"}</span>
                            <span class="section-divider"></span>
                            <span>{"ORIGIN STORY"}</span>
                        </p>
                        <h2 class="chapter-title">{ "The Chance Seedling" }</h2>
                        <p class="drop-cap">{ ambrosia::ORIGIN_STORY }</p>
                        <blockquote class="pull-quote">
                            { "\u{201C}Pickers, who seldom eat apples, loved Ambrosia, and stripped the tree before the rest of the orchard was ready.\u{201D}" }
                        </blockquote>
                        <p class="signature">{ "\u{2014} Wilfrid Mennell, Cawston, BC" }</p>
                    </div>
                    <div class="chapter-visual-col reveal-scale">
                        <div class="orchard-image">
                            <div class="orchard-placeholder">
                                <span>{"\u{1F333}"}</span>
                                <p>{ "Mennell Orchard" }</p>
                                <p class="orchard-location">{ "Similkameen Valley, British Columbia" }</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
