mod app;
mod db;
mod util;

use crate::app::PrehniteApp;
use prehnite_core::db::{initialize_db, DBType, DatabaseError};
use prehnite_core::i18n::initialize_i18n_from_db;
use prehnite_core::log::initialize_logger;
use prehnite_core::settings::registry::SettingRegistry;
use prehnite_core::settings::GlobalSettingKey;
use prehnite_core::util::alert::{
    fatal_initialize_app_error_db, fatal_initialize_setting_registry_error,
};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use tracing::error;
use prehnite_font_manager::get_default_font_family;

#[derive(Debug)]
enum InitializeError {
    DatabaseError(DatabaseError),
    LoadSettingRegistry,
}

impl Display for InitializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for InitializeError {}

impl From<DatabaseError> for InitializeError {
    fn from(value: DatabaseError) -> Self {
        InitializeError::DatabaseError(value)
    }
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
        initialize_logger();
        initialize_db().await?;
        if !SettingRegistry::load(DBType::AppGlobal).await {
            return Err(InitializeError::LoadSettingRegistry);
        };
        initialize_i18n_from_db().await?;
        if let None =
            SettingRegistry::get(&GlobalSettingKey::Font.into()).and_then(|v| v.get::<String>())
        {
            SettingRegistry::immediate_apply(GlobalSettingKey::Font.into(), get_default_font_family().into()).await;
        }
        Ok(())
    }

    func().await.unwrap_or_else(|e| {
        let err_msg = format!("{:#?}", e);
        error!("{}", err_msg);
        match e {
            InitializeError::DatabaseError(e) => {
                fatal_initialize_app_error_db(e).show().unwrap();
            }
            InitializeError::LoadSettingRegistry => {
                fatal_initialize_setting_registry_error().show().unwrap();
            }
        }
        panic!()
    });
}

fn main() -> Result<(), iced::Error> {
    initializer();
    PrehniteApp::run()
}
