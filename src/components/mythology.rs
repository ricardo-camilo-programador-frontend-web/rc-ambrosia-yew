use rust_i18n::t;
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
                            <span>{ t!("mythology.label") }</span>
                        </p>
                        <h2 class="chapter-title chapter-title-light">{ t!("mythology.title") }</h2>
                        <p class="mythology-body">{ t!("mythology.body") }</p>
                        <blockquote class="mythology-quote">{ t!("mythology.quote") }</blockquote>
                        <div class="mythology-seal">{ t!("mythology.seal") }</div>
                    </div>
                    <div class="mythology-visual reveal-scale">
                        <div class="statue-placeholder"><span>{ "\u{1F3DB}\u{FE0F}" }</span></div>
                    </div>
                </div>
            </section>
        }
    }
}
