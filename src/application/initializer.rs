use crate::application::settings::{fetch_all_settings, SupportedLanguages, APPLIED_REGISTRY};
use dioxus_i18n::prelude::{use_init_i18n, I18nConfig};
use dioxus_i18n::unic_langid::LanguageIdentifier;

pub async fn load_settings() -> anyhow::Result<()> {
    let registry = fetch_all_settings().await?;
    APPLIED_REGISTRY.with(|x| *x.borrow_mut() = registry);
    Ok(())
}

fn get_current_locale() -> LanguageIdentifier {
    APPLIED_REGISTRY.with(|x| x.borrow().get_locale().into())
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
