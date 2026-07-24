use rust_i18n::t;
use yew::prelude::*;

pub struct Nutrition;

impl Component for Nutrition {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="chapter dark-section nutrition-section" id="nutrition">
                <div class="nutrition-overlay"></div>
                <div class="chapter-wide nutrition-content">
                    <p class="section-label section-label-light">
                        <span class="section-number">{"07"}</span>
                        <span class="section-divider"></span>
                        <span>{ t!("nutrition.label") }</span>
                    </p>
                    <h2 class="chapter-title chapter-title-light reveal">{ t!("nutrition.title") }</h2>
                    <div class="stat-grid">
                        <article class="stat-card reveal-scale">
                            <strong class="stat-value">{ "52" }</strong>
                            <p class="stat-label">{ t!("nutrition.kcal") }</p>
                        </article>
                        <article class="stat-card reveal-scale">
                            <strong class="stat-value">{ "2.4" }</strong>
                            <p class="stat-label">{ t!("nutrition.fiber") }</p>
                        </article>
                        <article class="stat-card reveal-scale">
                            <strong class="stat-value">{ "4.6" }</strong>
                            <p class="stat-label">{ t!("nutrition.vitc") }</p>
                        </article>
                        <article class="stat-card reveal-scale">
                            <strong class="stat-value">{ t!("nutrition.ethylene_low") }</strong>
                            <p class="stat-label">{ t!("nutrition.ethylene_label") }</p>
                        </article>
                        <article class="stat-card reveal-scale">
                            <strong class="stat-value">{ t!("nutrition.antioxidants_high") }</strong>
                            <p class="stat-label">{ t!("nutrition.antioxidants_label") }</p>
                        </article>
                    </div>
                </div>
            </section>
        }
    }
}
