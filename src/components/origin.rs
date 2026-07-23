use crate::data::ambrosia;
use yew::prelude::*;

#[allow(clippy::module_inception)]
pub struct Origin;

impl Component for Origin {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="chapter" id="origin">
                <div class="chapter-inner">
                    <p class="chapter-subtitle reveal">{ "The Chance Seedling" }</p>
                    <h2 class="chapter-title reveal">{ "A Serendipitous Discovery" }</h2>
                    <p class="drop-cap reveal">{ ambrosia::ORIGIN_STORY }</p>
                    <blockquote class="pull-quote reveal">
                        { "\u{201C}Pickers, who seldom eat apples, loved Ambrosia, and stripped the tree before the rest of the orchard was ready.\u{201D}" }
                    </blockquote>
                </div>
            </section>
        }
    }
}
