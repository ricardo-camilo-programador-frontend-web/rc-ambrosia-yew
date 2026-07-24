use rust_i18n::t;
use crate::i18n::I18nContext;
use yew::prelude::*;

#[allow(clippy::module_inception)]
pub struct Origin(yew::ContextHandle<I18nContext>);

impl Component for Origin {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let handle = ctx.link().context::<I18nContext>(ctx.link().callback(|_: I18nContext| ()))
            .map(|(_, h)| h)
            .expect("I18nContext must be provided");
        Self(handle)
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
                    <figure class="chapter-visual-col orchard-image reveal-scale relative overflow-hidden rounded-2xl shadow-editorial">
                        <img src="/images/ambrosia-hero.jpg" alt="Fresh red apples harvested together" />
                        <figcaption>
                            <strong>{ t!("origin.orchard") }</strong>
                            <span>{ t!("origin.location") }</span>
                        </figcaption>
                    </figure>
                </div>
            </section>
        }
    }
}
