use prehnite_core::constants::global_db_file_path;
use prehnite_core::db::migrate::{app_global, prehnite_book};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, Sqlite, SqlitePool};
use std::path::PathBuf;
use std::sync::{LazyLock, OnceLock, RwLock};
use std::time::Duration;
use tracing::log::LevelFilter;
use tracing_unwrap::{OptionExt, ResultExt};

static GLOBAL_DB_POOL: OnceLock<SqlitePool> = OnceLock::new();
static BOOK_DB_POOL: LazyLock<RwLock<Option<SqlitePool>>> = LazyLock::new(|| RwLock::new(None));

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The global database file path does not exist.")]
    MissingGlobalDatabaseFilePath,
}

fn connect_option(path: PathBuf) -> SqliteConnectOptions {
    SqliteConnectOptions::default()
        .filename(path)
        .foreign_keys(true)
        .create_if_missing(true)
        .log_slow_statements(LevelFilter::Warn, Duration::from_millis(300))
        .log_statements(LevelFilter::Trace)
}

pub async fn acquire_global() -> anyhow::Result<PoolConnection<Sqlite>> {
    Ok(GLOBAL_DB_POOL
        .get()
        .expect_or_log("GLOBAL_DB_POOL has not been initialized.")
        .acquire()
        .await?)
}

pub async fn acquire_book() -> anyhow::Result<Option<PoolConnection<Sqlite>>> {
    if is_book_opened() {
        Ok(Some(
            BOOK_DB_POOL
                .read()
                .unwrap()
                .as_ref()
                .unwrap()
                .acquire()
                .await?,
        ))
    } else {
        Ok(None)
    }
}

async fn connect_pool(path: PathBuf) -> sqlx::Result<SqlitePool> {
    Ok(SqlitePool::connect_with(connect_option(path)).await?)
}

pub async fn initialize_global_db_pool() -> anyhow::Result<()> {
    if GLOBAL_DB_POOL.get().is_none() {
        let pool = connect_pool(global_db_file_path().ok_or(Error::MissingGlobalDatabaseFilePath)?)
            .await?;
        app_global::migrate(&pool).await?;
        GLOBAL_DB_POOL.set(pool).ok();
    }
    Ok(())
}

pub async fn open_book_db_pool(path: impl Into<PathBuf>) -> sqlx::Result<()> {
    close_book_db_pool().await;
    let pool = connect_pool(path.into()).await?;
    prehnite_book::migrate(&pool).await?;
    *BOOK_DB_POOL.write().unwrap() = Some(pool);
    Ok(())
}

pub async fn close_book_db_pool() {
    if is_book_opened() {
        BOOK_DB_POOL
            .read()
            .unwrap_or_log()
            .as_ref()
            .unwrap()
            .close()
            .await;
        *BOOK_DB_POOL.write().unwrap_or_log() = None;
    }
}

pub fn is_book_opened() -> bool {
    BOOK_DB_POOL.read().unwrap_or_log().is_some()
}
