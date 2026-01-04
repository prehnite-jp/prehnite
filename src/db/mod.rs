#![allow(unused)]
pub mod error;
pub mod schema;
mod util;

use sqlx::SqlitePool;

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

pub async fn migrate(
    pool: &SqlitePool,
    mode: MigrateMode,
) -> Result<(), sqlx::migrate::MigrateError> {
    match mode {
        MigrateMode::PrehniteBook => &migrate::prehnite_book::MIGRATOR,
        MigrateMode::AppGlobal => &migrate::app_global::MIGRATOR,
    }
    .run(pool)
    .await
}
