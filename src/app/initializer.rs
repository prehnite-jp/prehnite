use crate::app::settings::{fetch_all_settings, get_applied, SupportedLanguages};
use dioxus_i18n::prelude::{use_init_i18n, I18nConfig};
use dioxus_i18n::unic_langid::LanguageIdentifier;
use tracing_unwrap::ResultExt;

pub async fn load_settings() -> anyhow::Result<()> {
    let registry = fetch_all_settings().await?;
    *get_applied().write().unwrap_or_log() = registry;
    Ok(())
}

fn get_current_locale() -> LanguageIdentifier {
    get_applied().read().unwrap_or_log().get_locale().into()
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
