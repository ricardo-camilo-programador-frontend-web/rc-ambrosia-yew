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
                // Persist to sessionStorage
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
        let items = ambrosia::CULINARY_USES
            .iter()
            .enumerate()
            .map(|(i, (icon, title, desc))| {
                let is_open = self.open_index == Some(i);
                let content_class = if is_open {
                    "accordion-content open"
                } else {
                    "accordion-content"
                };
                html! {
                    <div class="accordion-item reveal">
                        <button
                            class="accordion-header"
                            onclick={ctx.link().callback(move |_| CulinaryMsg::Toggle(i))}
                            aria-expanded={is_open.to_string()}
                        >
                            <span>{ icon }</span>
                            <span>{ title }</span>
                        </button>
                        <div class={content_class}>
                            <p style="padding: 1rem 0;">{ desc }</p>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();

        html! {
            <section class="chapter" id="culinary">
                <div class="chapter-inner">
                    <p class="chapter-subtitle reveal">{ "How To Enjoy" }</p>
                    <h2 class="chapter-title reveal">{ "Culinary Delights" }</h2>
                    <div class="accordion">
                        { items }
                    </div>
                </div>
            </section>
        }
    }
}
