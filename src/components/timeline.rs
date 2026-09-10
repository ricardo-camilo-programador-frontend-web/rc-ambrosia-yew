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

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        crate::hooks::use_sticky_scroll::setup_sticky_scroll();
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let step_cards = ambrosia::TIMELINE_EVENTS
            .iter()
            .map(|(year, title, desc)| {
                html! {
                    <div class="scroll-step">
                        <div class="timeline-year">{ year }</div>
                        <h3 class="timeline-event-title">{ title }</h3>
                        <p class="timeline-desc">{ desc }</p>
                    </div>
                }
            })
            .collect::<Html>();

        let timeline_dots = ambrosia::TIMELINE_EVENTS
            .iter()
            .map(|(year, _title, _desc)| {
                html! {
                    <li class="timeline-item">
                        <div class="timeline-dot"></div>
                        <div class="timeline-year">{ year }</div>
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
                    <div class="sticky-scroll-container">
                        <div class="sticky-scroll-figure">
                            <ol class="timeline-track">{ timeline_dots }</ol>
                        </div>
                        <div class="sticky-scroll-steps">{ step_cards }</div>
                    </div>
                </div>
            </section>
        }
    }
}
