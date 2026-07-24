use rust_i18n::t;
use yew::prelude::*;

use crate::i18n::I18nContext;

pub struct Navbar {
    languages: Vec<(&'static str, &'static str)>,
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
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LanguageChanged(lang) => {
                if let Some((_ctx, handle)) = ctx.link().context::<I18nContext>(Callback::noop()) {
                    _ctx.set_locale(&lang);
                }
                true // Force re-render
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get current locale from context
        let current_locale = ctx.link()
            .context::<I18nContext>(Callback::noop())
            .map(|(_ctx, _handle)| _ctx.locale.clone())
            .unwrap_or_else(|| "en".to_string());

        let lang_options = self.languages.iter().map(|(code, label)| {
            html! { <option value={*code}>{ *label }</option> }
        }).collect::<Html>();

        html! {
            <nav class="navbar" role="navigation" aria-label="Main navigation">
                <span class="navbar-brand">{"\u{1F34E} Ambrosia"}</span>
                <ul class="navbar-links">
                    <li><a href="#origin">{ t!("nav.origin") }</a></li>
                    <li><a href="#timeline">{ t!("nav.timeline") }</a></li>
                    <li><a href="#characteristics">{ t!("nav.characteristics") }</a></li>
                    <li><a href="#mythology">{ t!("nav.mythology") }</a></li>
                    <li><a href="#nutrition">{ t!("nav.nutrition") }</a></li>
                    <li><a href="#series">{ t!("nav.series") }</a></li>
                </ul>
                <div class="lang-switcher">
                    <select aria-label="Select language"
                        value={current_locale}
                        onchange={ctx.link().callback(|e: Event| {
                            let sel = e.target_dyn_into::<web_sys::HtmlSelectElement>();
                            let lang = sel.and_then(|s| Some(s.value())).unwrap_or_else(|| "en".to_string());
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
}
