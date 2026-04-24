use dioxus_fullstack::get;
use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};
use easy_settings::Registry;
use prehnite_core::db::schema::Setting;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing_unwrap::ResultExt;

#[get("/api/settings")]
pub async fn fetch_all_settings() -> Result<GlobalSettings, anyhow::Error> {
    let mut conn = crate::backend::db::acquire_global().await?;
    let mut result = GlobalSettings::default();
    result.set_from_row_vec(
        Setting::select_all(&mut *conn)
            .await?
            .into_iter()
            .map(|x| x.to_setting_row())
            .collect(),
    );
    Ok(result)
}

#[cfg(feature = "desktop")]
thread_local! {
    pub static APPLIED_REGISTRY: std::cell::RefCell<GlobalSettings> = std::cell::RefCell::new(GlobalSettings::default());
}

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
