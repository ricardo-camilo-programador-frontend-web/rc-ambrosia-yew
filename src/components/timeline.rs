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
            .enumerate()
            .map(|(_i, (year, title, desc))| {
                html! {
                    <li class="timeline-item reveal">
                        <div class="timeline-dot"></div>
                        <div class="timeline-year">{ year }</div>
                        <div class="timeline-event-title">{ title }</div>
                        <p class="timeline-desc">{ desc }</p>
                    </li>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter dark-section timeline-section" id="timeline">
                <div class="chapter-wide">
                    <p class="section-label section-label-light">
                        <span class="section-number">{"03"}</span>
                        <span class="section-divider"></span>
                        <span>{"TIMELINE"}</span>
                    </p>
                    <h2 class="chapter-title chapter-title-light reveal">{ "A Journey" }</h2>
                    <ol class="timeline-track">
                        { events }
                    </ol>
                </div>
            </section>
        }
    }
}
