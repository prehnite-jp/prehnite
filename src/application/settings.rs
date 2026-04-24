use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};
use easy_settings::Registry;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::{LazyLock, RwLock};
use tracing_unwrap::ResultExt;

pub static APPLIED_REGISTRY: LazyLock<RwLock<GlobalSettings>> =
    LazyLock::new(|| RwLock::new(Default::default()));

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum SupportedLanguages {
    #[default]
    #[serde(rename = "en-US")]
    EnUS,
    #[serde(rename = "ja-JP")]
    JaJP,
}

impl SupportedLanguages {
    fn get_locale_default() -> Self {
        sys_locale::get_locale()
            .and_then(|x| serde_json::from_str(&format!("\"{}\"", x)).ok())
            .unwrap_or_default()
    }
}

impl From<&SupportedLanguages> for &'static str {
    fn from(value: &SupportedLanguages) -> Self {
        match value {
            SupportedLanguages::EnUS => "en-US",
            SupportedLanguages::JaJP => "ja-JP",
        }
    }
}

impl From<SupportedLanguages> for &'static str {
    fn from(value: SupportedLanguages) -> Self {
        (&value).into()
    }
}

impl From<SupportedLanguages> for LanguageIdentifier {
    fn from(value: SupportedLanguages) -> Self {
        LanguageIdentifier::from_str(value.into())
            .ok_or_log()
            .unwrap_or_else(|| langid!("en-US"))
    }
}

#[derive(Clone, Registry, Debug, Deserialize, Serialize)]
#[easy_settings(categories("settings_category_general"))]
pub struct GlobalSettings {
    #[easy_settings(default = SupportedLanguages::get_locale_default())]
    #[easy_settings(categories("settings_category_general"))]
    locale: Option<SupportedLanguages>,
    // TODO: デフォルトのフォントファミリーを指定する。
    #[easy_settings(categories("settings_category_general"))]
    font: Option<String>,
    #[easy_settings(categories("settings_category_general"))]
    last_opened_file: Option<String>,
    #[easy_settings(default = true)]
    #[easy_settings(categories("settings_category_general"))]
    auto_open_last_opened_file: Option<bool>,
}
