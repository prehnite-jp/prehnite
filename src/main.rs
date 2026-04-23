#![cfg_attr(feature = "release", windows_subsystem = "windows")]
pub mod backend;
pub mod frontend;

use prehnite_core::db::{initialize_db, DBType, DatabaseError};
use prehnite_core::font::get_default_font_family;
use prehnite_core::i18n::initialize_i18n_from_settings;
use prehnite_core::log::initialize_logger;
use prehnite_core::settings;
use prehnite_core::util::alert::{
    fatal_initialize_app_error, fatal_initialize_setting_registry_error,
};
use std::fmt::Debug;
use std::sync::LockResult;
use thiserror::Error;
use tracing::error;
use tracing_unwrap::ResultExt;

#[derive(Debug, Error)]
enum InitializeError {
    #[error("initialize error. because database")]
    DatabaseError(#[from] DatabaseError),
    #[error("initialize error. because can not load setting registry")]
    LoadSettingRegistry,
    #[error("failed to initialize log rotate.")]
    InitializeLogRotateConfig(#[from] prehnite_core::log::InitError),
}

impl From<sqlx::Error> for InitializeError {
    fn from(value: sqlx::Error) -> Self {
        DatabaseError::from(value).into()
    }
}

#[tokio::main]
#[tracing::instrument]
async fn initializer() {
    async fn func() -> Result<(), InitializeError> {
        initialize_logger()?;
        initialize_db().await?;
        match settings::get_global().write().ok_or_log().as_mut() {
            Some(x) => {
                x.load_all().await?;
            }
            None => return Err(InitializeError::LoadSettingRegistry),
        };
        initialize_i18n_from_settings().await?;
        // if let None =
        //     SettingRegistry::get(&GlobalSettingKey::Font.into()).and_then(|v| v.get::<String>())
        // {
        //     SettingRegistry::immediate_apply(
        //         GlobalSettingKey::Font.into(),
        //         get_default_font_family().into(),
        //     )
        //     .await?;
        // }
        Ok(())
    }

    func().await.unwrap_or_else(|e| {
        let err_msg = format!("{:#?}", e);
        error!("{}", err_msg);
        match e {
            InitializeError::DatabaseError(_) | InitializeError::InitializeLogRotateConfig(_) => {
                fatal_initialize_app_error(e).show().unwrap_or_log();
            }
            InitializeError::LoadSettingRegistry => {
                fatal_initialize_setting_registry_error()
                    .show()
                    .unwrap_or_log();
            }
        }
        panic!()
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    initializer();

    Ok(())
}
