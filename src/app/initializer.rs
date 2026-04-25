use crate::app::settings::{get_settings, SupportedLanguages};
use dioxus_i18n::prelude::{use_init_i18n, I18nConfig};
use dioxus_i18n::unic_langid::LanguageIdentifier;

fn get_current_locale() -> LanguageIdentifier {
    get_settings().get_locale().into()
}

pub fn initialize_i18n() {
    use_init_i18n(|| {
        I18nConfig::new(get_current_locale())
            .with_locale((
                SupportedLanguages::EnUS.into(),
                include_str!("../../assets/locales/en-US.ftl"),
            ))
            .with_locale((
                SupportedLanguages::JaJP.into(),
                include_str!("../../assets/locales/ja-JP.ftl"),
            ))
    });
}
