#![allow(unused)]
pub mod error;
pub mod schema;
mod util;

use crate::db::migrate::migrate;
use crate::util::app_global::global_dir;
use chrono::Duration;
use log::LevelFilter;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Sqlite, SqlitePool};
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum DBType {
    PrehniteBook,
    AppGlobal,
}

pub mod migrate {
    use crate::db::DBType;
    use sqlx::SqlitePool;

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

pub struct Database {
    app_global_db_pool: Arc<Mutex<Pool>>,
    prehnite_book_db_pool: Arc<Mutex<Pool>>,
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
        if cfg!(debug_assertions) {
            "app_global.db".into()
        } else {
            let mut db_file = global_dir();
            db_file.push("app_global.db");
            db_file
        }
    }

    fn connect_option(file: impl AsRef<Path>) -> SqliteConnectOptions {
        SqliteConnectOptions::default()
            .filename(file)
            .foreign_keys(true)
            .create_if_missing(true)
            .log_slow_statements(
                LevelFilter::Debug,
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
            app_global_db_pool: Arc::new(Mutex::new(Pool::new(Some(pool)))),
            prehnite_book_db_pool: Arc::new(Mutex::new(Pool::new(None))),
        })
    }

    pub async fn open_book(&mut self, path: impl AsRef<Path>) -> Result<(), DatabaseError> {
        let mut pool = SqlitePoolOptions::new()
            .connect_with(Self::connect_option(path))
            .await?;

        migrate(&mut pool, DBType::PrehniteBook).await?;

        self.prehnite_book_db_pool.lock().await.set_pool(Some(pool));
        Ok(())
    }

    pub async fn acquire(
        &self,
        mode: DBType,
    ) -> Result<Option<PoolConnection<Sqlite>>, DatabaseError> {
        Ok(match mode {
            DBType::PrehniteBook => {
                match self.prehnite_book_db_pool.clone().lock().await.get_pool() {
                    None => None,
                    Some(v) => Some(v.acquire().await?),
                }
            }
            DBType::AppGlobal => match self.app_global_db_pool.clone().lock().await.get_pool() {
                None => None,
                Some(v) => Some(v.acquire().await?),
            },
        })
    }
}
