#![allow(unused)]
pub mod error;
pub mod query;
pub mod schema;
mod util;

use crate::db::migrate::migrate;
use crate::on_error_logging;
use crate::util::alert::{alert_i18n_show, UnwrapOrErrorAlert};
use crate::util::app_global::global_dir;
use chrono::Duration;
use log::LevelFilter;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Sqlite, SqlitePool};
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::error;

impl UnwrapOrErrorAlert<PoolConnection<Sqlite>> for Option<PoolConnection<Sqlite>> {
    fn unwrap_or_alert(self) -> PoolConnection<Sqlite> {
        self.unwrap_or_else(|| {
            alert_i18n_show(("error", "cant-connect-database"));
            panic!()
        })
    }
}

#[derive(Clone, Debug)]
pub enum DBType {
    PrehniteBook,
    AppGlobal,
}

impl From<DBType> for String {
    fn from(value: DBType) -> Self {
        match value {
            DBType::PrehniteBook => "Prehnite book",
            DBType::AppGlobal => "App global settings",
        }
        .into()
    }
}

impl Display for DBType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

pub mod migrate {
    use crate::db::DBType;
    use sqlx::SqlitePool;

    pub mod prehnite_book {
        use sqlx::migrate::Migrator;
        use sqlx::sqlx_macros::migrate;

        pub static MIGRATOR: Migrator = migrate!("migrations/prehnite_book");
    }

    pub mod app_global {
        use sqlx::migrate::Migrator;
        use sqlx::sqlx_macros::migrate;

        pub static MIGRATOR: Migrator = migrate!("migrations/app_global");
    }

    pub async fn migrate(
        pool: &SqlitePool,
        mode: DBType,
    ) -> Result<(), sqlx::migrate::MigrateError> {
        match mode {
            DBType::PrehniteBook => &prehnite_book::MIGRATOR,
            DBType::AppGlobal => &app_global::MIGRATOR,
        }
        .run(pool)
        .await
    }
}

#[derive(Debug)]
struct Pool {
    pool: Option<SqlitePool>,
}

impl Pool {
    fn new(pool: Option<SqlitePool>) -> Self {
        Self { pool }
    }

    fn set_pool(&mut self, pool: Option<SqlitePool>) {
        self.pool = pool;
    }

    fn get_pool(&self) -> &Option<SqlitePool> {
        &self.pool
    }
}

pub async fn initialize_db() -> Result<(), DatabaseError> {
    let v = Database::initialize().await?;
    DATABASE.set(Arc::new(RwLock::new(v)));
    Ok(())
}

static DATABASE: OnceLock<Arc<RwLock<Database>>> = OnceLock::new();

#[tracing::instrument]
pub fn get_database() -> Arc<RwLock<Database>> {
    match DATABASE.get() {
        None => {
            error!("Failed to get database. The database may not be initialized.");
            panic!();
        }
        Some(v) => v,
    }
    .clone()
}

#[tracing::instrument]
pub async fn acquire_err_handled(mode: DBType) -> Option<PoolConnection<Sqlite>> {
    match get_database().read().await.acquire(mode.clone()).await {
        Ok(v) => match v {
            None => {
                error!("{} Database not connected.", mode);
                None
            }
            Some(v) => Some(v),
        },
        Err(e) => {
            error!("Failed to acquire {} Database. Error: {:#?}", mode, e);
            None
        }
    }
}

#[derive(Debug)]
pub struct Database {
    app_global_db_pool: Arc<RwLock<Pool>>,
    prehnite_book_db_pool: Arc<RwLock<Pool>>,
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
    fn get_app_global_database_url() -> PathBuf {
        let mut db_file = global_dir();
        db_file.push("app_global.db");
        db_file
    }

    fn connect_option(file: impl AsRef<Path>) -> SqliteConnectOptions {
        SqliteConnectOptions::default()
            .filename(file)
            .foreign_keys(true)
            .create_if_missing(true)
            .log_slow_statements(
                LevelFilter::Warn,
                Duration::milliseconds(300).to_std().unwrap(),
            )
            .log_statements(LevelFilter::Trace)
    }

    pub async fn initialize() -> Result<Self, DatabaseError> {
        let app_global_path = Self::get_app_global_database_url();
        let mut pool = SqlitePoolOptions::new()
            .connect_with(Self::connect_option(app_global_path))
            .await?;

        migrate(&mut pool, DBType::AppGlobal).await?;

        Ok(Self {
            app_global_db_pool: Arc::new(RwLock::new(Pool::new(Some(pool)))),
            prehnite_book_db_pool: Arc::new(RwLock::new(Pool::new(None))),
        })
    }

    #[tracing::instrument]
    pub async fn open_book(&mut self, path: impl AsRef<Path> + Debug) -> Result<(), DatabaseError> {
        let pool_result = SqlitePoolOptions::new()
            .connect_with(Self::connect_option(path))
            .await;

        on_error_logging!(pool_result);
        let mut pool = pool_result?;

        migrate(&mut pool, DBType::PrehniteBook).await?;

        self.prehnite_book_db_pool
            .write()
            .await
            .set_pool(Some(pool));
        Ok(())
    }

    pub async fn acquire(
        &self,
        mode: DBType,
    ) -> Result<Option<PoolConnection<Sqlite>>, DatabaseError> {
        Ok(match mode {
            DBType::PrehniteBook => {
                match self.prehnite_book_db_pool.clone().read().await.get_pool() {
                    None => None,
                    Some(v) => Some(v.acquire().await?),
                }
            }
            DBType::AppGlobal => match self.app_global_db_pool.clone().read().await.get_pool() {
                None => None,
                Some(v) => Some(v.acquire().await?),
            },
        })
    }
}
