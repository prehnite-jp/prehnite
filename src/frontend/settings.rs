use serde::{Deserialize, Serialize};
use easy_settings::Registry;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum SupportedLanguages {
    #[default]
    EnUS,
    JaJP,
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

#[derive(Clone, Registry, Debug, Deserialize, Serialize)]
#[easy_settings(categories("settings_category_general"))]
pub struct GlobalSettings {
    #[easy_settings(default = SupportedLanguages::default())]
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
