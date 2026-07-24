use rust_i18n::t;
use yew::prelude::*;

pub struct Hero;

impl Component for Hero {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        crate::hooks::use_scroll_reveal::setup_scroll_reveal();
        crate::hooks::use_reading_progress::setup_reading_progress();
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="hero dark-section" id="hero">
                <div class="hero-overlay"></div>
                <div class="hero-content">
                    <div class="hero-text reveal">
                        <p class="section-label section-label-light">
                            <span class="section-number">{"01"}</span>
                            <span class="section-divider"></span>
                            <span>{ t!("hero.label") }</span>
                        </p>
                        <h1 class="hero-title">{ "Ambrosia" }</h1>
                        <p class="hero-subtitle">{ t!("hero.subtitle") }</p>
                        <p class="hero-tagline">{ t!("hero.tagline") }</p>
                        <a href="#origin" class="hero-cta">{ t!("hero.cta") }</a>
                    </div>
                    <div class="hero-visual reveal-scale">
                        <div class="hero-apple">{ "\u{1F34E}" }</div>
                    </div>
                </div>
                <div class="scroll-indicator">
                    <div class="scroll-line"></div>
                    <span>{ t!("hero.scroll") }</span>
                </div>
            </section>
        }
    }
}
