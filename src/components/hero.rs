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
            <section class="hero" id="hero">
                <div class="hero-apple reveal-scale">{"🍎"}</div>
                <h1 class="hero-title reveal">{ "Ambrosia" }</h1>
                <p class="hero-subtitle reveal">{ "Food of the Gods" }</p>
                <p class="hero-tagline reveal">
                    { "A chance seedling from British Columbia, named after the mythical food that granted immortality." }
                </p>
                <div class="scroll-indicator">{ "Scroll to explore ↓" }</div>
            </section>
        }
    }
}
