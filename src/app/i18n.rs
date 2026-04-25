use crate::app::settings::get_settings;
use dioxus_i18n::prelude::i18n;

pub fn apply_language_from_settings() {
    let lang = get_settings().get_locale();
    i18n().set_language(lang.into());
}
