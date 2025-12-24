#![allow(unused)]
pub mod error;
pub mod schema;
pub mod app_global_schema;
mod util;

use sqlx::migrate::Migrator;
use sqlx::sqlx_macros::migrate;
use sqlx::SqlitePool;
use std::path::PathBuf;

fn app_global_migrator() -> Migrator {
    migrate!("./migrations_app_global")
}

fn book_migrator() -> Migrator {
    migrate!("./migrations_prehnite_book")
}

pub enum MigrateMode {
    PrehniteBook,
    AppGlobal,
}

pub async fn migrate(
    conn: &SqlitePool,
    mode: MigrateMode,
    file_path: impl Into<PathBuf>,
) -> Result<(), sqlx::migrate::MigrateError> {
    match mode {
        MigrateMode::PrehniteBook => book_migrator(),
        MigrateMode::AppGlobal => app_global_migrator(),
    }
        .run(conn)
        .await
}
