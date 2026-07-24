use rust_i18n::t;
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
                            <span>{ t!("origin.label") }</span>
                        </p>
                        <h2 class="chapter-title">{ t!("origin.title") }</h2>
                        <p class="drop-cap">{ t!("origin.story") }</p>
                        <blockquote class="pull-quote">{ t!("origin.quote") }</blockquote>
                        <p class="signature">{ t!("origin.signature") }</p>
                    </div>
                    <div class="chapter-visual-col reveal-scale">
                        <div class="orchard-image">
                            <div class="orchard-placeholder">
                                <span>{ "\u{1F333}" }</span>
                                <p>{ t!("origin.orchard") }</p>
                                <p class="orchard-location">{ t!("origin.location") }</p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
