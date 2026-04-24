use crate::app::db::acquire_global;
use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};
use easy_settings::Registry;
use prehnite_core::db::schema::Setting;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::{Arc, LazyLock, RwLock};
use tracing_unwrap::ResultExt;

static APPLIED_REGISTRY: LazyLock<Arc<RwLock<GlobalSettings>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Default::default())));

pub fn get_applied() -> Arc<RwLock<GlobalSettings>> {
    APPLIED_REGISTRY.clone()
}

static CACHED_REGISTRY: LazyLock<tokio::sync::RwLock<GlobalSettings>> =
    LazyLock::new(|| tokio::sync::RwLock::new(Default::default()));

pub async fn fetch_all_settings() -> anyhow::Result<GlobalSettings> {
    let mut conn = acquire_global().await?;
    let mut result = GlobalSettings::default();
    result.set_from_row_vec(
        Setting::select_all(&mut *conn)
            .await?
            .into_iter()
            .map(|x| x.to_setting_row())
            .collect(),
    );
    *CACHED_REGISTRY.write().await = result.clone();
    Ok(result)
}

pub async fn save_all_settings(settings: GlobalSettings) -> anyhow::Result<()> {
    use sqlx::Acquire;
    let mut conn = acquire_global().await?;
    let mut tx = conn.begin().await?;
    {
        let cached = CACHED_REGISTRY.read().await;
        for (key, val) in settings
            .items()
            .iter()
            .filter(|x| cached.get(x.0).unwrap() != x.1)
        {
            sqlx::query("INSERT INTO settings(setting_key, setting_value) VALUES (?1, ?2) ON CONFLICT DO UPDATE SET setting_value = ?2")
                .bind(key.to_string())
                .bind(val.raw_string())
                .execute(&mut *tx)
                .await?;
        }
    }
    tx.commit().await?;
    *CACHED_REGISTRY.write().await = settings;
    Ok(())
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
