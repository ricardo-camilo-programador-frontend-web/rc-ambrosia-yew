use crate::data::ambrosia;
use crate::i18n::I18nContext;
use rust_i18n::t;
use yew::prelude::*;

#[allow(dead_code)]
pub struct Timeline(yew::ContextHandle<I18nContext>);

impl Component for Timeline {
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
        let events = ambrosia::TIMELINE_EVENTS
            .iter()
            .map(|(year, title, desc)| {
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
                        <span>{ t!("timeline.label") }</span>
                    </p>
                    <h2 class="chapter-title chapter-title-light reveal">{ t!("timeline.title") }</h2>
                    <ol class="timeline-track">{ events }</ol>
                </div>
            </section>
        }
    }
}
