#![doc = "データベースのマイグレーション"]
use sqlx::SqlitePool;
use crate::db::connection::DBType;

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
pub async fn migrate(pool: &SqlitePool, mode: DBType) -> Result<(), sqlx::migrate::MigrateError> {
    match mode {
        DBType::PrehniteBook => &prehnite_book::MIGRATOR,
        DBType::AppGlobal => &app_global::MIGRATOR,
    }
    .run(pool)
    .await
}
