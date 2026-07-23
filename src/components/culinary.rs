use crate::data::ambrosia;
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
        let items = ambrosia::CULINARY_USES.iter().enumerate().map(|(i, (_icon, title, desc))| {
            let is_open = self.open_index == Some(i);
            let content_class = if is_open { "accordion-content open" } else { "accordion-content" };
            html! {
                <div class="accordion-card reveal">
                    <button class="accordion-header"
                        onclick={ctx.link().callback(move |_| CulinaryMsg::Toggle(i))}
                        aria-expanded={is_open.to_string()}>
                        <span class="accordion-number">{ format!("{}", i + 1) }</span>
                        <span class="accordion-title">{ title }</span>
                        <span class={if is_open { "accordion-icon open" } else { "accordion-icon" }}>{ "+" }</span>
                    </button>
                    <div class={content_class}>
                        <p>{ desc }</p>
                    </div>
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
                            <span>{"CULINARY"}</span>
                        </p>
                        <h2 class="chapter-title chapter-title-light">{ "How To Enjoy" }</h2>
                        <div class="accordion">{ items }</div>
                    </div>
                    <div class="culinary-visual-col">
                        <div class="culinary-visual-placeholder">
                            <span>{ "\u{1F36F}\u{FE0F}" }</span>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}
