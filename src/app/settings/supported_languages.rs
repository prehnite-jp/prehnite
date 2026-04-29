use std::str::FromStr;
use easy_settings::re_export::serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr, VariantArray};
use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};
use tracing_unwrap::ResultExt;

#[derive(
    Clone, Debug, Default, Deserialize, Serialize, VariantArray, IntoStaticStr, Display, PartialEq,
)]
pub enum SupportedLanguages {
    #[default]
    #[strum(serialize = "en-US")]
    #[serde(rename = "en-US")]
    EnUS,
    #[strum(serialize = "ja-JP")]
    #[serde(rename = "ja-JP")]
    JaJP,
}

impl SupportedLanguages {
    pub fn get_locale_default() -> Self {
        sys_locale::get_locale()
            .and_then(|x| serde_json::from_str(&format!("\"{}\"", x)).ok())
            .unwrap_or_default()
    }
}

impl From<SupportedLanguages> for LanguageIdentifier {
    fn from(value: SupportedLanguages) -> Self {
        LanguageIdentifier::from_str(value.into())
            .ok_or_log()
            .unwrap_or_else(|| langid!("en-US"))
    }
}