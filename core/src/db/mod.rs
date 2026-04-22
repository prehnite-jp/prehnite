#![allow(unused)]
#![doc = "アプリケーションのデータベース"]
pub mod query;
pub mod schema;
mod util;

use crate::db::migrate::migrate;
use crate::settings;
use crate::settings::GlobalSettings;
use crate::util::alert::{alert_i18n_show, alert_i18n_spawn, UnwrapOrErrorAlert};
use crate::util::app_global::global_dir;
use crate::util::file_dialog::OpenPrehniteBookStatus;
use chrono::Duration;
use easy_settings::sqlite::SettingManager;
use log::LevelFilter;
use native_dialog::MessageLevel;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{ConnectOptions, Sqlite, SqlitePool};
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, LockResult, OnceLock, RwLockWriteGuard};
use strum::{EnumString, IntoStaticStr};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::error;
use tracing_unwrap::ResultExt;

impl UnwrapOrErrorAlert<PoolConnection<Sqlite>> for Option<PoolConnection<Sqlite>> {
    fn unwrap_or_alert(self) -> PoolConnection<Sqlite> {
        self.unwrap_or_else(|| {
            alert_i18n_show(("error", "cant-connect-database"), MessageLevel::Error);
            panic!()
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString, IntoStaticStr)]
pub enum DBType {
    #[strum(serialize = "Prehnite book")]
    PrehniteBook,
    #[strum(serialize = "App global settings")]
    AppGlobal,
}

/// データベースのマイグレーション
pub mod migrate {
    use crate::db::DBType;
    use sqlx::SqlitePool;

    /// ブックファイル
    pub mod prehnite_book {
        use sqlx::migrate::Migrator;
        use sqlx::sqlx_macros::migrate;

        /// マイグレーション定義
        pub static MIGRATOR: Migrator = migrate!("../migrations/prehnite_book");
    }

    /// アプリのグローバルデータベース
    pub mod app_global {
        use sqlx::migrate::Migrator;
        use sqlx::sqlx_macros::migrate;

        /// マイグレーション定義
        pub static MIGRATOR: Migrator = migrate!("../migrations/app_global");
    }

    /// マイグレーションを実行します。
    /// # Parameters
    /// - `pool` マイグレーションを実行するDB接続
    /// - `mode` 初期化したいデータベースのタイプ
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
    pool: Option<Arc<SqlitePool>>,
}

impl Pool {
    fn new(pool: Option<SqlitePool>) -> Self {
        Self {
            pool: pool.map(Arc::new),
        }
    }

    fn set_pool(&mut self, pool: Option<SqlitePool>) {
        self.pool = pool.map(Arc::new);
    }

    fn get_pool(&self) -> Option<Arc<SqlitePool>> {
        self.pool.clone()
    }
}

/// グローバルデータベース接続を初期化します。
pub async fn initialize_db() -> Result<(), DatabaseError> {
    let v = Database::initialize().await?;
    DATABASE.set(Arc::new(RwLock::new(v)));
    Ok(())
}

static DATABASE: OnceLock<Arc<RwLock<Database>>> = OnceLock::new();

#[tracing::instrument]
fn get_database() -> Arc<RwLock<Database>> {
    match DATABASE.get() {
        None => {
            error!("Failed to get database. The database may not be initialized.");
            panic!();
        }
        Some(v) => v.clone(),
    }
}

/// グローバルデータベース接続を取得します。
pub async fn acquire_conn(mode: DBType) -> Result<Option<PoolConnection<Sqlite>>, DatabaseError> {
    get_database().read().await.acquire(mode.clone()).await
}

/// グローバルデータベースプールを取得します。
pub async fn get_pool(mode: DBType) -> Option<Arc<SqlitePool>> {
    get_database().read().await.get_pool(mode).await
}

#[tracing::instrument]
/// グローバルデータベース接続を取得します。エラーが発生した場合はログに出力します。
pub async fn acquire_or_log(mode: DBType) -> Option<PoolConnection<Sqlite>> {
    match acquire_conn(mode.clone()).await.ok_or_log()? {
        Some(v) => Some(v),
        None => {
            error!("{} Database not connected.", <&'static str>::from(mode));
            None
        }
    }
}

/// グローバルデータベース接続を取得します。エラーが発生した場合はログに出力し、アラートを表示します。
pub async fn acquire_book_or_alert() -> PoolConnection<Sqlite> {
    acquire_or_log(DBType::PrehniteBook).await.unwrap_or_alert()
}

#[tracing::instrument]
/// Prehniteブックファイルを開きます。エラーが発生した場合はログに出力し、アラートを表示します。
pub fn open_book_or_alert(book_path: PathBuf) -> bool {
    let x = settings::get_global();
    match x.write().ok_or_log().as_mut() {
        Some(x) => {
            x.get_tmp_registry()
                .set_last_opened_file(book_path.to_str().map(|x| x.to_string()));
        }
        None => {
            alert_i18n_show(("error", "book-open-error"), MessageLevel::Error);
            return false;
        }
    }
    true
}

/// Prehniteブックファイルを閉じます。エラーが発生した場合はログに出力します。
pub async fn close_book_or_log() {
    get_database()
        .write()
        .await
        .prehnite_book_db_pool
        .write()
        .await
        .set_pool(None);
    if let Some(x) = settings::get_global().write().ok_or_log().as_mut() {
        x.get_tmp_registry().set_last_opened_file(None);
        x.save_and_apply().await;
    };
}

static IS_PREHNITE_BOOK_OPENED: LazyLock<Arc<std::sync::RwLock<DbOpenedStatus>>> =
    LazyLock::new(|| Arc::new(std::sync::RwLock::new(DbOpenedStatus::default())));

struct DbOpenedStatus(bool);

impl Default for DbOpenedStatus {
    fn default() -> Self {
        Self(false)
    }
}

impl DbOpenedStatus {
    fn set(&mut self, v: bool) {
        self.0 = v;
    }
}

#[derive(Debug)]
/// グローバルデータベース接続の構造体
pub struct Database {
    app_global_db_pool: RwLock<Pool>,
    prehnite_book_db_pool: RwLock<Pool>,
}

#[derive(Error, Debug)]
/// グローバルデータベース接続のエラー
pub enum DatabaseError {
    #[error("Failed to execute statement.")]
    DBError(#[from] sqlx::Error),
    #[error("Failed to execute database migrations.")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error("Failed to decode item_type.")]
    ItemTypeDecodeError,
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

    /// 接続を初期化し、マイグレーションを実行します。
    pub async fn initialize() -> Result<Self, DatabaseError> {
        let app_global_path = Self::get_app_global_database_url();
        let mut pool = SqlitePoolOptions::new()
            .connect_with(Self::connect_option(app_global_path))
            .await?;

        migrate(&mut pool, DBType::AppGlobal).await?;

        Ok(Self {
            app_global_db_pool: RwLock::new(Pool::new(Some(pool))),
            prehnite_book_db_pool: RwLock::new(Pool::new(None)),
        })
    }

    #[tracing::instrument]
    /// Prehniteブックを新しく開き、マイグレーションを実行します。
    pub async fn open_book(&mut self, path: impl AsRef<Path> + Debug) -> Result<(), DatabaseError> {
        let pool_result = SqlitePoolOptions::new()
            .connect_with(Self::connect_option(path))
            .await;

        let mut pool = pool_result?;

        migrate(&mut pool, DBType::PrehniteBook).await?;

        self.prehnite_book_db_pool
            .write()
            .await
            .set_pool(Some(pool));
        IS_PREHNITE_BOOK_OPENED.write().unwrap_or_log().set(true);
        Ok(())
    }

    /// データベース接続を取得します。
    pub async fn acquire(
        &self,
        mode: DBType,
    ) -> Result<Option<PoolConnection<Sqlite>>, DatabaseError> {
        Ok(match mode {
            DBType::PrehniteBook => match self.prehnite_book_db_pool.read().await.get_pool() {
                None => None,
                Some(v) => Some(v.acquire().await?),
            },
            DBType::AppGlobal => match self.app_global_db_pool.read().await.get_pool() {
                None => None,
                Some(v) => Some(v.acquire().await?),
            },
        })
    }

    /// Prehniteブックが開かれているか否か。
    pub fn is_book_opened() -> bool {
        IS_PREHNITE_BOOK_OPENED
            .read()
            .map(|v| v.0)
            .unwrap_or_default()
    }

    pub async fn get_pool(&self, mode: DBType) -> Option<Arc<SqlitePool>> {
        match mode {
            DBType::PrehniteBook => self.prehnite_book_db_pool.read().await.get_pool(),
            DBType::AppGlobal => self.app_global_db_pool.read().await.get_pool(),
        }
    }
}
