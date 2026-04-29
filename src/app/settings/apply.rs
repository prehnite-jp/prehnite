use crate::app::settings::global_settings::GlobalSettings;
use dioxus::document::eval;
use dioxus_desktop::window;
use dioxus_i18n::prelude::i18n;
use tracing_unwrap::ResultExt;

pub fn apply_language_from_settings(settings: &GlobalSettings) {
    i18n().set_language(settings.get_locale().into());
}

pub async fn apply_theme(registry: &GlobalSettings) {
    let theme = registry.get_theme();
    window().set_theme(Some(theme.clone().into()));
    eval(&format!(
        "document.documentElement.setAttribute(\"data-theme\", \"{}\");",
        theme.clone()
    ))
    .await
    .ok_or_log();
}
