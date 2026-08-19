use yew::prelude::*;

use crate::components::{
    ambrosia_gold::AmbrosiaGold, characteristics::Characteristics, culinary::Culinary,
    footer::Footer, hero::Hero, mythology::Mythology, navbar::Navbar, nutrition::Nutrition,
    origin::Origin, parentage::Parentage, regions::Regions, series::Series, timeline::Timeline,
};
use crate::i18n::I18nContext;

pub struct App {
    locale: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let locale = Self::load_saved_locale();
        rust_i18n::set_locale(&locale);
        Self { locale }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetLocale(lang) => {
                rust_i18n::set_locale(&lang);
                Self::save_locale(&lang);

                if let Some(window) = web_sys::window() {
                    let dir = if lang == "ar" { "rtl" } else { "ltr" };
                    if let Some(doc) = window.document() {
                        if let Some(html) = doc.document_element() {
                            let _ = html.set_attribute("dir", dir);
                        }
                    }
                }

                self.locale = lang;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let setter = ctx.link().callback(Msg::SetLocale);
        let i18n_ctx = I18nContext::new(self.locale.clone(), setter);

        html! {
            <ContextProvider<I18nContext> context={i18n_ctx}>
                <a href="#article-content" class="skip-link">{"Skip to content"}</a>
                <div class="reading-progress" id="reading-progress" style="width: 0%"></div>
                <Navbar />
                <main id="article-content">
                    <Hero />
                    <Origin />
                    <Timeline />
                    <Characteristics />
                    <Mythology />
                    <Parentage />
                    <Nutrition />
                    <Regions />
                    <Culinary />
                    <AmbrosiaGold />
                    <Series />
                </main>
                <Footer />
            </ContextProvider<I18nContext>>
        }
    }
}

impl App {
    fn load_saved_locale() -> String {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(lang)) = storage.get_item("ambrosia-lang") {
                    if !lang.is_empty() {
                        return lang;
                    }
                }
            }
        }
        "en".to_string()
    }

    fn save_locale(lang: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("ambrosia-lang", lang);
            }
        }
    }
}

pub enum Msg {
    SetLocale(String),
}
