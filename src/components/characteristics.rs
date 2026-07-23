use crate::data::ambrosia;
use yew::prelude::*;

pub struct Characteristics;

impl Component for Characteristics {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let items = ambrosia::CHARACTERISTICS
            .iter()
            .map(|(_icon, title, desc)| {
                html! {
                    <article class="char-list-item reveal-left">
                        <span class="char-dot">{ "\u{25CF}" }</span>
                        <div>
                            <h3 class="char-heading">{ title }</h3>
                            <p class="char-text">{ desc }</p>
                        </div>
                    </article>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter dark-section char-section" id="characteristics">
                <div class="chapter-grid-two char-grid-layout">
                    <div class="char-list-col">
                        <p class="section-label section-label-light">
                            <span class="section-number">{"04"}</span>
                            <span class="section-divider"></span>
                            <span>{"THE APPLE"}</span>
                        </p>
                        <h2 class="chapter-title chapter-title-light">{ "Meet Ambrosia" }</h2>
                        <div class="char-list">{ items }</div>
                    </div>
                    <div class="char-visual-col reveal-scale">
                        <p class="char-slogan">{ "Beauty you can see." }</p>
                        <p class="char-slogan">{ "Quality you can feel." }</p>
                        <div class="apple-diagram">
                            <div class="apple-large">{ "\u{1F34E}" }</div>
                            <div class="annotation-line annotation-1"></div>
                            <div class="annotation-label annotation-label-1">
                                <strong>{ "FLESH" }</strong>
                                <span>{ "Cream, firm and juicy" }</span>
                            </div>
                            <div class="annotation-line annotation-2"></div>
                            <div class="annotation-label annotation-label-2">
                                <strong>{ "RESISTANCE" }</strong>
                                <span>{ "to oxidation. Slow to brown." }</span>
                            </div>
                            <div class="annotation-line annotation-3"></div>
                            <div class="annotation-label annotation-label-3">
                                <strong>{ "SHAPE" }</strong>
                                <span>{ "Conical, elegant, symmetrical" }</span>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
