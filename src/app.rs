use yew::prelude::*;

/// Root App component for the Ambrosia apple landing page.
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="app">
            <section class="hero">
                <h1>{"🍎 Ambrosia"}</h1>
                <p class="tagline">{"Food of the Gods"}</p>
                <p class="subtitle">
                    {"A chance seedling from British Columbia, named after the mythical food that granted immortality."}
                </p>
            </section>
        </main>
    }
}
