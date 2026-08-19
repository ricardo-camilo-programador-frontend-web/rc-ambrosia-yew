pub mod app;
pub mod components;
pub mod data;
pub mod hooks;
pub mod i18n;

rust_i18n::i18n!("src/i18n/locales", fallback = "en");
