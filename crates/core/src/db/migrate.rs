#![doc = "データベースのマイグレーション"]

/// ブックファイル
pub mod prehnite_book {
    use sqlx::SqlitePool;
    use sqlx::migrate::Migrator;
    use sqlx::sqlx_macros::migrate as sqlx_migrate;

    /// マイグレーション定義
    pub static MIGRATOR: Migrator = sqlx_migrate!("migrations/prehnite_book");

    /// マイグレーションを実行します。
    /// # Parameters
    /// - `pool` マイグレーションを実行するDB接続
    pub async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
        MIGRATOR.run(pool).await
    }
}

/// アプリのグローバルデータベース
pub mod app_global {
    use sqlx::SqlitePool;
    use sqlx::migrate::Migrator;
    use sqlx::sqlx_macros::migrate as sqlx_migrate;

    /// マイグレーション定義
    pub static MIGRATOR: Migrator = sqlx_migrate!("migrations/app_global");

    /// マイグレーションを実行します。
    /// # Parameters
    /// - `pool` マイグレーションを実行するDB接続
    pub async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
        MIGRATOR.run(pool).await
    }
}
