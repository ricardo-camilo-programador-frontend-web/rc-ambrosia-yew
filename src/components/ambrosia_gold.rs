use crate::i18n::I18nContext;
use rust_i18n::t;
use yew::prelude::*;

#[allow(dead_code)]
pub struct AmbrosiaGold(yew::ContextHandle<I18nContext>);

impl Component for AmbrosiaGold {
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
            <section class="chapter dark-section gold-section" id="ambrosia-gold">
                <div class="chapter-grid-two">
                    <div class="gold-text-col reveal-left">
                        <p class="section-label section-label-light">
                            <span class="section-number">{"10"}</span>
                            <span class="section-divider"></span>
                            <span>{ t!("gold.label") }</span>
                        </p>
                        <h2 class="gold-title reveal">{ t!("gold.title") }</h2>
                        <p class="gold-intro">{ t!("gold.intro") }</p>
                        <div class="gold-cards">
                            <article class="premium-card reveal">
                                <h3 class="premium-heading">{ t!("gold.club") }</h3>
                                <p>{ t!("gold.club_desc") }</p>
                            </article>
                            <article class="premium-card reveal">
                                <h3 class="premium-heading">{ t!("gold.patent") }</h3>
                                <p>{ t!("gold.patent_desc") }</p>
                            </article>
                            <article class="premium-card reveal">
                                <h3 class="premium-heading">{ t!("gold.market") }</h3>
                                <p>{ t!("gold.market_desc") }</p>
                            </article>
                        </div>
                    </div>
                    <figure class="gold-visual-col gold-photo reveal-scale relative overflow-hidden rounded-2xl shadow-editorial">
                        <img src="/images/ambrosia-orchard.jpg" alt="An Ambrosia apple in an orchard" />
                        <figcaption>{ "Ambrosia Gold" }</figcaption>
                    </figure>
                </div>
            </section>
        }
    }
}
