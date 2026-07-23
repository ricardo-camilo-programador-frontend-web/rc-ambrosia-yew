use crate::data::ambrosia;
use yew::prelude::*;

pub struct Timeline;

impl Component for Timeline {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let events = ambrosia::TIMELINE_EVENTS
            .iter()
            .map(|(year, title, desc)| {
                html! {
                    <div class="timeline-entry reveal">
                        <div class="timeline-card">
                            <div class="timeline-year">{ year }</div>
                            <h3>{ title }</h3>
                            <p>{ desc }</p>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter" id="timeline">
                <div class="chapter-wide">
                    <p class="chapter-subtitle reveal">{ "A Journey" }</p>
                    <h2 class="chapter-title reveal">{ "From Seedling to Stardom" }</h2>
                    <div class="timeline">
                        { events }
                    </div>
                </div>
            </section>
        }
    }
}
