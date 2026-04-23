use crate::db::{get_pool, DBType, DatabaseError};
use crate::i18n::SupportedLanguages;
use crate::i18n::DEFAULT_LANG_ID;
use crate::util::alert::fatal_initialize_app_error;
use easy_settings::sqlite::{SettingManager, SettingManagerBuilder};
use easy_settings::Registry;
use log::error;
use std::sync::{Arc, OnceLock, RwLock};
use thiserror::Error;
use tracing_unwrap::{OptionExt, ResultExt};

#[derive(Error, Debug)]
pub enum InitializeSettingManagerError {
    #[error("Database error")]
    GetPoolError(#[from] DatabaseError),
    #[error("Failed to execute statement.")]
    DBError(#[from] sqlx::Error),
}

#[tracing::instrument]
pub async fn initialize_global_setting_manager() -> Result<(), InitializeSettingManagerError> {
    let mut manager: SettingManager<GlobalSettings> = SettingManagerBuilder::default().tablename("settings", get_pool(DBType::AppGlobal).await.unwrap_or_else(|| {
        fatal_initialize_app_error("The settings manager cannot be initialized because the database connection has not been initialized.")
            .show().unwrap_or_log();
        error!("The settings manager cannot be initialized because the database connection has not been initialized.");
        panic!()
    })).build().unwrap_or_log();
    manager.load_all().await?;
    GLOBAL_SETTING_MANAGER
        .set(Arc::new(RwLock::new(manager)))
        .ok()
        .expect_or_log("Global Setting Manager already initialized.");
    Ok(())
}

static GLOBAL_SETTING_MANAGER: OnceLock<Arc<RwLock<SettingManager<GlobalSettings>>>> =
    OnceLock::new();

pub fn get_global() -> Arc<RwLock<SettingManager<GlobalSettings>>> {
    match GLOBAL_SETTING_MANAGER.get() {
        None => {
            fatal_initialize_app_error("Global Setting Manager is not be initialized.This error occurs if you use the settings manager before it has been initialized.")
                .show()
                .unwrap_or_log();
            error!("Global Setting Manager is not be initialized.");
            panic!()
        }
        Some(x) => x.clone(),
    }
}

#[derive(Clone, Registry, Debug)]
#[easy_settings(categories("settings_category_general"))]
pub struct GlobalSettings {
    #[easy_settings(default = DEFAULT_LANG_ID.into())]
    #[easy_settings(categories("settings_category_general"))]
    locale: Option<SupportedLanguages>,
    // #[easy_settings(default = get_default_font_family().into())]
    #[easy_settings(categories("settings_category_general"))]
    font: Option<String>,
    #[easy_settings(categories("settings_category_general"))]
    last_opened_file: Option<String>,
    #[easy_settings(default = true)]
    #[easy_settings(categories("settings_category_general"))]
    auto_open_last_opened_file: Option<bool>,
}
