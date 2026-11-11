use rust_i18n::t;
use yew::prelude::*;

use crate::i18n::I18nContext;

pub struct Navbar {
    languages: Vec<(&'static str, &'static str)>,
    menu_open: bool,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            languages: vec![
                ("en", "\u{1F1EC}\u{1F1E7} EN"),
                ("zh-CN", "\u{1F1E8}\u{1F1F3} \u{4E2D}\u{6587}"),
                ("hi", "\u{1F1EE}\u{1F1F3} \u{0939}\u{093F}\u{0902}"),
                ("es", "\u{1F1EA}\u{1F1F8} ES"),
                ("fr", "\u{1F1EB}\u{1F1F7} FR"),
                ("ar", "\u{1F1F8}\u{1F1E6} \u{0639}"),
                ("bn", "\u{1F1E7}\u{1F1E9} \u{09AC}\u{09BE}\u{0982}"),
                ("ru", "\u{1F1F7}\u{1F1FA} RU"),
                ("pt-BR", "\u{1F1E7}\u{1F1F7} PT"),
                ("ur", "\u{1F1F5}\u{1F1F0} \u{0627}\u{0631}\u{062F}\u{0648}"),
                ("id", "\u{1F1EE}\u{1F1E9} ID"),
                ("de", "\u{1F1E9}\u{1F1EA} DE"),
                ("ja", "\u{1F1EF}\u{1F1F5} \u{65E5}"),
                ("sw", "\u{1F1F0}\u{1F1EA} SW"),
                ("te", "\u{1F1EE}\u{1F1F3} \u{0C24}\u{0C46}"),
                ("tr", "\u{1F1F9}\u{1F1F7} TR"),
                ("ko", "\u{1F1F0}\u{1F1F7} \u{D55C}"),
                ("ta", "\u{1F1EE}\u{1F1F3} \u{0BA4}\u{0BAE}"),
                ("mr", "\u{1F1EE}\u{1F1F3} \u{092E}\u{0930}\u{093E}"),
                ("pa", "\u{1F1EE}\u{1F1F3} \u{0A2A}\u{0A70}"),
            ],
            menu_open: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LanguageChanged(lang) => {
                if let Some((i18n, _handle)) = ctx.link().context::<I18nContext>(Callback::noop()) {
                    i18n.set_locale(&lang);
                }
                true
            }
            Msg::ToggleMenu => {
                self.menu_open = !self.menu_open;
                true
            }
            Msg::CloseMenu => {
                self.menu_open = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_locale = ctx
            .link()
            .context::<I18nContext>(Callback::noop())
            .map(|(i18n, _handle)| i18n.locale.clone())
            .unwrap_or_else(|| "en".to_string());

        let lang_options = self
            .languages
            .iter()
            .map(|(code, label)| {
                html! { <option value={*code}>{ *label }</option> }
            })
            .collect::<Html>();

        let close = ctx.link().callback(|_: MouseEvent| Msg::CloseMenu);
        let menu_class = if self.menu_open {
            "navbar-links is-open"
        } else {
            "navbar-links"
        };
        let toggle_label = if self.menu_open {
            t!("nav.close")
        } else {
            t!("nav.menu")
        };

        html! {
            <nav class="navbar" aria-label="Main navigation">
                <a href="#hero" class="navbar-brand">{"\u{1F34E} Ambrosia"}</a>
                <button
                    type="button"
                    class="nav-toggle"
                    aria-expanded={self.menu_open.to_string()}
                    aria-controls="primary-nav"
                    onclick={ctx.link().callback(|_| Msg::ToggleMenu)}
                >
                    <span class="nav-toggle-icon" aria-hidden="true"></span>
                    <span class="sr-only">{ toggle_label }</span>
                </button>
                <ul id="primary-nav" class={menu_class}>
                    <li><a href="#origin" onclick={close.clone()}>{ t!("nav.origin") }</a></li>
                    <li><a href="#timeline" onclick={close.clone()}>{ t!("nav.timeline") }</a></li>
                    <li><a href="#characteristics" onclick={close.clone()}>{ t!("nav.characteristics") }</a></li>
                    <li><a href="#mythology" onclick={close.clone()}>{ t!("nav.mythology") }</a></li>
                    <li><a href="#nutrition" onclick={close.clone()}>{ t!("nav.nutrition") }</a></li>
                    <li><a href="#series" onclick={close.clone()}>{ t!("nav.series") }</a></li>
                </ul>
                <div class="lang-switcher">
                    <select aria-label="Select language"
                        value={current_locale}
                        onchange={ctx.link().callback(|e: Event| {
                            let sel = e.target_dyn_into::<web_sys::HtmlSelectElement>();
                            let lang = sel.map(|s| s.value()).unwrap_or_else(|| "en".to_string());
                            Msg::LanguageChanged(lang)
                        })}>
                        { lang_options }
                    </select>
                </div>
            </nav>
        }
    }
}

pub enum Msg {
    LanguageChanged(String),
    ToggleMenu,
    CloseMenu,
}
