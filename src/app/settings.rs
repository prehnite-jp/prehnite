use crate::app::db::acquire_global;
use crate::app::i18n::apply_language_from_settings;
use crate::windows::main_window::menu;
use dark_light::Mode;
use dioxus::core::Task;
use dioxus::prelude::*;
use dioxus_desktop::window;
use dioxus_desktop::wry::RGBA;
use dioxus_i18n::unic_langid::{langid, LanguageIdentifier};
use easy_settings::Registry;
use prehnite_core::db::schema::Setting;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Duration;
use strum::{Display, IntoStaticStr, VariantArray};
use tracing_unwrap::ResultExt;

pub static THEME: GlobalSignal<Theme> = Signal::global(Theme::get_system_default);
static APPLIED_REGISTRY: GlobalSignal<GlobalSettings> = Signal::global(GlobalSettings::default);

pub fn use_setting_loader() {
    use_effect(move || {
        let _ = get_settings();
        apply_all_settings();
    });
}

pub fn apply_all_settings() -> Task {
    spawn(async {
        apply_language_from_settings();
        menu::main_window_menu_bar().apply_i18n();
        if !window().is_visible() {
            tokio::time::sleep(Duration::from_millis(100)).await;
            window().set_visible(true);
        }
        *THEME.write() = get_settings().read().get_theme();
    })
}

fn set_applied(registry: GlobalSettings) {
    *APPLIED_REGISTRY.write() = registry;
    apply_all_settings();
}

pub async fn load() -> anyhow::Result<()> {
    set_applied(fetch_all_settings().await?);
    Ok(())
}

pub fn get_settings() -> Signal<GlobalSettings> {
    APPLIED_REGISTRY.signal()
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
    load().await?;
    Ok(())
}

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
    fn get_locale_default() -> Self {
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

#[derive(
    Clone, Deserialize, Serialize, Debug, PartialEq, VariantArray, IntoStaticStr, Display, Default,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl From<Mode> for Theme {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Dark => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

impl From<Theme> for dioxus_desktop::tao::window::Theme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Light => dioxus_desktop::tao::window::Theme::Light,
            Theme::Dark => dioxus_desktop::tao::window::Theme::Dark,
        }
    }
}

impl Theme {
    fn get_system_default() -> Self {
        dark_light::detect()
            .ok_or_log()
            .map(|x| x.into())
            .unwrap_or_default()
    }

    pub fn bg_color(&self) -> RGBA {
        match self {
            Theme::Light => (0xFF, 0xFF, 0xFF, 0xFF),
            Theme::Dark => (0x3E, 0x3E, 0x3E, 0xFF),
        }
    }
}

#[derive(Clone, Registry, Debug, Deserialize, Serialize, PartialEq)]
#[easy_settings(categories("general"))]
pub struct GlobalSettings {
    #[easy_settings(default = SupportedLanguages::get_locale_default())]
    #[easy_settings(categories("general"))]
    locale: Option<SupportedLanguages>,
    #[easy_settings(categories("general"))]
    last_opened_file: Option<String>,
    #[easy_settings(default = true)]
    #[easy_settings(categories("general"))]
    auto_open_last_opened_file: Option<bool>,
    #[easy_settings(default = Theme::get_system_default())]
    #[easy_settings(categories("general"))]
    theme: Option<Theme>,
}

impl GlobalSettings {
    pub async fn fetch() -> anyhow::Result<Self> {
        let mut conn = acquire_global().await?;
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
}
