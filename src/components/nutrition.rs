use crate::data::ambrosia;
use yew::prelude::*;

pub struct Nutrition;

impl Component for Nutrition {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let stats = ambrosia::NUTRITION
            .iter()
            .map(|(value, label)| {
                html! {
                    <article class="stat-card reveal-scale">
                        <strong class="stat-value">{ value }</strong>
                        <p class="stat-label">{ label }</p>
                    </article>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter dark-section nutrition-section" id="nutrition">
                <div class="nutrition-overlay"></div>
                <div class="chapter-wide nutrition-content">
                    <p class="section-label section-label-light">
                        <span class="section-number">{"07"}</span>
                        <span class="section-divider"></span>
                        <span>{"NUTRITION"}</span>
                    </p>
                    <h2 class="chapter-title chapter-title-light reveal">{ "By The Numbers" }</h2>
                    <div class="stat-grid">{ stats }</div>
                </div>
            </section>
        }
    }
}
