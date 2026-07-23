use yew::prelude::*;

pub struct Navbar {
    languages: Vec<(&'static str, &'static str)>,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            languages: vec![
                ("en", "🇬🇧 EN"),
                ("zh-CN", "🇨🇳 中文"),
                ("hi", "🇮🇳 हिं"),
                ("es", "🇪🇸 ES"),
                ("fr", "🇫🇷 FR"),
                ("ar", "🇸🇦 ع"),
                ("bn", "🇧🇩 বাং"),
                ("ru", "🇷🇺 RU"),
                ("pt-BR", "🇧🇷 PT"),
                ("ur", "🇵🇰 اردو"),
                ("id", "🇮🇩 ID"),
                ("de", "🇩🇪 DE"),
                ("ja", "🇯🇵 日"),
                ("sw", "🇰🇪 SW"),
                ("te", "🇮🇳 తె"),
                ("tr", "🇹🇷 TR"),
                ("ko", "🇰🇷 한"),
                ("ta", "🇮🇳 தம"),
                ("mr", "🇮🇳 मरा"),
                ("pa", "🇮🇳 ਪੰ"),
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LanguageChanged(lang) => {
                rust_i18n::set_locale(&lang);
                if let Some(window) = web_sys::window() {
                    if let Some(storage) = window.local_storage().ok().flatten() {
                        let _ = storage.set_item("ambrosia-lang", &lang);
                    }
                    if lang == "ar" {
                        if let Some(doc) = window.document() {
                            if let Some(html) = doc.document_element() {
                                let _ = html.set_attribute("dir", "rtl");
                            }
                        }
                    } else {
                        if let Some(doc) = window.document() {
                            if let Some(html) = doc.document_element() {
                                let _ = html.set_attribute("dir", "ltr");
                            }
                        }
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let lang_options = self
            .languages
            .iter()
            .map(|(code, label)| {
                html! { <option value={*code}>{ *label }</option> }
            })
            .collect::<Html>();

        html! {
            <nav class="navbar" role="navigation" aria-label="Main navigation">
                <span class="navbar-brand">{"🍎 Ambrosia"}</span>
                <ul class="navbar-links">
                    <li><a href="#origin">{"Origin"}</a></li>
                    <li><a href="#timeline">{"Timeline"}</a></li>
                    <li><a href="#characteristics">{"The Apple"}</a></li>
                    <li><a href="#mythology">{"Mythology"}</a></li>
                    <li><a href="#nutrition">{"Nutrition"}</a></li>
                    <li><a href="#series">{"Series"}</a></li>
                </ul>
                <div class="lang-switcher">
                    <select
                        aria-label="Select language"
                        onchange={ctx.link().callback(|e: Event| {
                            let select = e.target_dyn_into::<web_sys::HtmlSelectElement>();
                            let lang = select.and_then(|s| Some(s.value())).unwrap_or("en".to_string());
                            Msg::LanguageChanged(lang)
                        })}
                    >
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
