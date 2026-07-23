use crate::data::ambrosia;
use yew::prelude::*;

pub struct Mythology;

impl Component for Mythology {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="mythology" id="mythology">
                <div class="chapter-inner" style="position: relative; z-index: 1;">
                    <p class="chapter-subtitle reveal">{ "Food of the Gods" }</p>
                    <h2 class="chapter-title reveal">{ "The Divine Connection" }</h2>
                    <p class="drop-cap reveal">
                        { "In ancient Greek mythology, ambrosia (\u{1F00}\u{3BC}\u{3B2}\u{3C1}\u{3BF}\u{3C3}\u{3AF}\u{3B1}) was the food of the Olympian gods. Those who consumed it were said to gain immortality \u{2014} a divine sustenance reserved for the mightiest beings in the pantheon. When Wilfrid Mennell tasted the apple that had mysteriously appeared in his orchard, its honeyed sweetness and ethereal crunch evoked this ancient myth. He named it Ambrosia, forging a bridge between the mythic past and a modern agricultural miracle." }
                    </p>
                    <blockquote class="pull-quote reveal">
                        { ambrosia::MYTHOLOGY_QUOTE }
                    </blockquote>
                </div>
            </section>
        }
    }
}
