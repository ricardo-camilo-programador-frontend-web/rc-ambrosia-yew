use rust_i18n::t;
use yew::prelude::*;

pub struct Culinary {
    open_index: Option<usize>,
}

pub enum CulinaryMsg {
    Toggle(usize),
}

impl Component for Culinary {
    type Message = CulinaryMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            open_index: Some(0),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CulinaryMsg::Toggle(index) => {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.session_storage() {
                        let new_idx = if self.open_index == Some(index) {
                            None
                        } else {
                            Some(index)
                        };
                        let _ = storage.set_item(
                            "ambrosia-accordion",
                            &new_idx.map(|i| i.to_string()).unwrap_or_default(),
                        );
                    }
                }
                self.open_index = if self.open_index == Some(index) {
                    None
                } else {
                    Some(index)
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let keys = [
            "culinary.fresh_title",
            "culinary.fresh_desc",
            "culinary.salads_title",
            "culinary.salads_desc",
            "culinary.baking_title",
            "culinary.baking_desc",
            "culinary.cider_title",
            "culinary.cider_desc",
            "culinary.cheese_title",
            "culinary.cheese_desc",
        ];
        let items: Vec<(String, String)> = (0..5)
            .map(|i| (t!(keys[i * 2]).to_string(), t!(keys[i * 2 + 1]).to_string()))
            .collect();

        let cards = items.into_iter().enumerate().map(|(i, (title, desc))| {
            let is_open = self.open_index == Some(i);
            let cls = if is_open { "accordion-content open" } else { "accordion-content" };
            html! {
                <div class="accordion-card reveal">
                    <button class="accordion-header"
                        onclick={ctx.link().callback(move |_| CulinaryMsg::Toggle(i))}
                        aria-expanded={is_open.to_string()}>
                        <span class="accordion-number">{ format!("{}", i + 1) }</span>
                        <span class="accordion-title">{ title }</span>
                        <span class={if is_open { "accordion-icon open" } else { "accordion-icon" }}>{ "+" }</span>
                    </button>
                    <div class={cls}><p>{ desc }</p></div>
                </div>
            }
        }).collect::<Html>();

        html! {
            <section class="chapter dark-section culinary-section" id="culinary">
                <div class="culinary-overlay"></div>
                <div class="chapter-grid-two culinary-content">
                    <div class="culinary-list-col">
                        <p class="section-label section-label-light">
                            <span class="section-number">{"09"}</span>
                            <span class="section-divider"></span>
                            <span>{ t!("culinary.label") }</span>
                        </p>
                        <h2 class="chapter-title chapter-title-light">{ t!("culinary.title") }</h2>
                        <div class="accordion">{ cards }</div>
                    </div>
                    <div class="culinary-visual-col">
                        <div class="culinary-visual-placeholder"><span>{ "\u{1F36F}\u{FE0F}" }</span></div>
                    </div>
                </div>
            </section>
        }
    }
}
