#![allow(unused)]
pub mod error;
pub mod schema;
mod util;

use crate::db::migrate::migrate;
use crate::fatal_init_db_error;
use crate::util::fatal_init_db_error;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Sqlite, SqlitePool};
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
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

    fn connect_option(file: impl AsRef<Path>) -> SqliteConnectOptions {
        SqliteConnectOptions::default()
            .filename(file)
            .foreign_keys(true)
            .create_if_missing(true)
    }

    pub async fn initialize() -> Result<Self, DatabaseError> {
        let app_global_path = std::env::var_os("APP_GLOBAL_DATABASE_PATH")
            .unwrap_or_else(Self::get_app_global_database_url);
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
