use yew::prelude::*;

/// I18n context: holds the current locale and a setter that triggers re-render.
#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub locale: String,
    setter: Callback<String>,
}

impl I18nContext {
    pub fn new(locale: String, setter: Callback<String>) -> Self {
        Self { locale, setter }
    }

    pub fn set_locale(&self, lang: &str) {
        self.setter.emit(lang.to_string());
    }
}
