use crate::data::ambrosia;
use rust_i18n::t;
use crate::i18n::I18nContext;
use yew::prelude::*;

pub struct Characteristics(yew::ContextHandle<I18nContext>);

impl Component for Characteristics {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let handle = ctx.link().context::<I18nContext>(ctx.link().callback(|_: I18nContext| ()))
            .map(|(_, h)| h)
            .expect("I18nContext must be provided");
        Self(handle)
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
                            <span>{ t!("characteristics.label") }</span>
                        </p>
                        <h2 class="chapter-title chapter-title-light">{ t!("characteristics.title") }</h2>
                        <div class="char-list">{ items }</div>
                    </div>
                    <div class="char-visual-col reveal-scale relative rounded-2xl">
                        <p class="char-slogan">{ t!("characteristics.slogan1") }</p>
                        <p class="char-slogan">{ t!("characteristics.slogan2") }</p>
                        <div class="apple-diagram">
                            <img class="apple-large" src="/images/ambrosia-detail.jpg" alt="A ripe red Ambrosia apple" />
                            <div class="annotation-line annotation-1"></div>
                            <div class="annotation-label annotation-label-1">
                                <strong>{ t!("characteristics.flesh") }</strong>
                                <span>{ t!("characteristics.flesh_desc") }</span>
                            </div>
                            <div class="annotation-line annotation-2"></div>
                            <div class="annotation-label annotation-label-2">
                                <strong>{ t!("characteristics.resistance") }</strong>
                                <span>{ t!("characteristics.resistance_desc") }</span>
                            </div>
                            <div class="annotation-line annotation-3"></div>
                            <div class="annotation-label annotation-label-3">
                                <strong>{ t!("characteristics.shape") }</strong>
                                <span>{ t!("characteristics.shape_desc") }</span>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
