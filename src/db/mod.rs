#![allow(unused)]
pub mod error;
pub mod schema;
mod util;

use crate::util::fatal_init_db_error;
use crate::fatal_init_db_error;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};

pub mod migrate {
    pub mod prehnite_book {
        use sqlx::migrate::Migrator;
        use sqlx::sqlx_macros::migrate;

        pub static MIGRATOR: Migrator = migrate!("./migrations/prehnite_book");
    }

    pub mod app_global {
        use sqlx::migrate::Migrator;
        use sqlx::sqlx_macros::migrate;

        pub static MIGRATOR: Migrator = migrate!("./migrations/app_global");
    }
}

pub enum MigrateMode {
    PrehniteBook,
    AppGlobal,
}

async fn migrate(pool: &SqlitePool, mode: MigrateMode) -> Result<(), sqlx::migrate::MigrateError> {
    match mode {
        MigrateMode::PrehniteBook => &migrate::prehnite_book::MIGRATOR,
        MigrateMode::AppGlobal => &migrate::app_global::MIGRATOR,
    }
    .run(pool)
    .await
}

pub struct Database {
    pub app_global_db_pool: SqlitePool,
    pub prehnite_book_db_pool: Option<SqlitePool>,
}

#[derive(Debug)]
pub enum DatabaseError {
    DBError(sqlx::Error),
    MigrateError(sqlx::migrate::MigrateError),
}

impl std::error::Error for DatabaseError {}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<sqlx::Error> for DatabaseError {
    fn from(value: sqlx::Error) -> Self {
        DatabaseError::DBError(value)
    }
}

impl From<sqlx::migrate::MigrateError> for DatabaseError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        DatabaseError::MigrateError(value)
    }
}

impl Database {
    fn get_app_global_database_url() -> OsString {
        if cfg!(debug_assertions) {
            "dev.db".into()
        } else {
            let mut db_file = std::env::home_dir().unwrap_or_else(|| {
                fatal_init_db_error!();
            });
            db_file.push(".jp.prehnite.prehnite");
            db_file.push("app_global.db");
            db_file
                .to_str()
                .unwrap_or_else(|| {
                    fatal_init_db_error!();
                })
                .into()
        }
    }

    pub async fn initialize() -> Result<Self, DatabaseError> {
        let app_global_path = std::env::var_os("APP_GLOBAL_DATABASE_PATH")
            .unwrap_or_else(Self::get_app_global_database_url);
        let mut pool = SqlitePoolOptions::new()
            .connect_with(
                SqliteConnectOptions::default()
                    .filename(app_global_path)
                    .foreign_keys(true)
                    .create_if_missing(true),
            )
            .await?;

        let mut self_res = Self {
            app_global_db_pool: pool,
            prehnite_book_db_pool: None,
        };

        self_res.migrate().await?;

        Ok(self_res)
    }

    async fn migrate(&mut self) -> Result<(), sqlx::migrate::MigrateError> {
        migrate(&mut self.app_global_db_pool, MigrateMode::AppGlobal).await
    }
}
