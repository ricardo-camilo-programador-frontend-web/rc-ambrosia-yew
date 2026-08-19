use crate::data::ambrosia;
use crate::i18n::I18nContext;
use rust_i18n::t;
use yew::prelude::*;

#[allow(dead_code)]
pub struct Parentage(yew::ContextHandle<I18nContext>);

impl Component for Parentage {
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
        let pollinators = ambrosia::POLLINATORS
            .iter()
            .map(|p| {
                html! { <li class="pill-tag">{ p }</li> }
            })
            .collect::<Html>();

        html! {
            <section class="chapter light-section" id="parentage">
                <div class="chapter-wide">
                    <p class="section-label">
                        <span class="section-number">{"06"}</span>
                        <span class="section-divider"></span>
                        <span>{ t!("parentage.label") }</span>
                    </p>
                    <h2 class="chapter-title reveal">{ t!("parentage.title") }</h2>
                    <p class="chapter-intro reveal">{ t!("parentage.intro") }</p>
                    <div class="parentage-diagram reveal-scale">
                        <div class="parent-row">
                            <div class="parent-node">
                                <span class="parent-role">{ t!("parentage.father_role") }</span>
                                <strong>{ ambrosia::PARENTAGE_FATHER }</strong>
                            </div>
                            <div class="parent-node">
                                <span class="parent-role">{ t!("parentage.mother_role") }</span>
                                <strong>{ ambrosia::PARENTAGE_MOTHER }</strong>
                            </div>
                        </div>
                        <div class="parent-connectors">
                            <div class="connector-line left"></div>
                            <div class="connector-line right"></div>
                        </div>
                        <div class="parent-node child-node">
                            <span class="parent-role">{ t!("parentage.result_role") }</span>
                            <strong class="child-name"><img class="parent-apple-photo" src="/images/ambrosia-gold.jpg" alt="" />{ "Ambrosia" }</strong>
                        </div>
                    </div>
                    <div class="pollinator-section reveal">
                        <p class="pollinator-label">{ t!("parentage.pollinators") }</p>
                        <ul class="pill-list">{ pollinators }</ul>
                    </div>
                </div>
            </section>
        }
    }
}
