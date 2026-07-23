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
                    <div class="stat-card reveal-scale">
                        <div class="stat-value">{ value }</div>
                        <div class="stat-label">{ label }</div>
                    </div>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter" id="nutrition">
                <div class="chapter-wide">
                    <p class="chapter-subtitle reveal">{ "By The Numbers" }</p>
                    <h2 class="chapter-title reveal">{ "Nutrition" }</h2>
                    <div class="nutrition-grid">
                        { stats }
                    </div>
                </div>
            </section>
        }
    }
}
