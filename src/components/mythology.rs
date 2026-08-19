use crate::i18n::I18nContext;
use rust_i18n::t;
use yew::prelude::*;

#[allow(dead_code)]
pub struct Mythology(yew::ContextHandle<I18nContext>);

impl Component for Mythology {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let handle = ctx
            .link()
            .context::<I18nContext>(ctx.link().callback(|_: I18nContext| ()))
            .map(|(_, h)| h)
            .expect("I18nContext must be provided");
        Self(handle)
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
                    <figure class="mythology-visual reveal-scale relative overflow-hidden rounded-2xl shadow-editorial">
                        <img src="/images/ambrosia-gold.jpg" alt="A red apple presented against a clean background" />
                    </figure>
                </div>
            </section>
        }
    }
}
