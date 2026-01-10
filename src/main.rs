mod app;
mod util;
mod widget;

use crate::app::PrehniteApp;
use prehnite_core::db::{get_database, initialize_db, DBType, DatabaseError};
use prehnite_core::i18n::initialize_i18n_from_db;
use prehnite_core::log::initialize_logger;
use prehnite_core::util::alert::fatal_initialize_app_error_db;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use tracing::error;

#[derive(Debug)]
enum InitializeError {
    DatabaseError(DatabaseError),
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
        initialize_i18n_from_db(
            get_database()
                .read()
                .await
                .acquire(DBType::AppGlobal)
                .await?
                .unwrap()
                .as_mut(),
        )
        .await?;
        Ok(())
    }

    func().await.unwrap_or_else(|e| {
        let err_msg = format!("{:#?}", e);
        error!("{}", err_msg);
        match e {
            InitializeError::DatabaseError(e) => {
                fatal_initialize_app_error_db(e).show().unwrap();
            }
        }
        panic!()
    });
}

fn main() -> Result<(), iced::Error> {
    initializer();
    PrehniteApp::run()
}
