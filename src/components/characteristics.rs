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
        let cards = ambrosia::CHARACTERISTICS
            .iter()
            .map(|(icon, title, desc)| {
                html! {
                    <div class="char-card reveal-scale">
                        <div class="char-card-icon">{ icon }</div>
                        <h3 class="char-card-title">{ title }</h3>
                        <p class="char-card-desc">{ desc }</p>
                    </div>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter" id="characteristics">
                <div class="chapter-wide">
                    <p class="chapter-subtitle reveal">{ "Meet Ambrosia" }</p>
                    <h2 class="chapter-title reveal">{ "The Apple" }</h2>
                    <div class="char-grid">
                        { cards }
                    </div>
                </div>
            </section>
        }
    }
}
