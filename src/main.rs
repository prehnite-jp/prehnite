mod app;

use crate::app::PrehniteApp;
use prehnite::db::{DBType, Database, DatabaseError};
use prehnite::i18n::{initialize_i18n_from_db};
use prehnite::log::initialize_logger;
use prehnite::util::fatal_initialize_app_error;

fn initialize_db() -> Result<Database, DatabaseError> {
    #[tokio::main]
    async fn func() -> Result<Database, DatabaseError> {
        let v = Database::initialize().await?;
        Ok(v)
    }
    func()
}

fn initialize_i18n(db: &mut Database) -> Result<(), DatabaseError> {
    #[tokio::main]
    async fn func(db: &mut Database) -> Result<(), DatabaseError> {
        initialize_i18n_from_db(db.acquire(DBType::AppGlobal).await?.unwrap().as_mut()).await?;
        Ok(())
    }
    func(db)
}

fn main() {
    initialize_logger();
    let mut db = initialize_db().unwrap_or_else(|e| {
        fatal_initialize_app_error(e);
        panic!()
    });
    initialize_i18n(&mut db).unwrap_or_else(|e| {
        fatal_initialize_app_error(e);
        panic!()
    });
    let app = PrehniteApp::new(db);
}
