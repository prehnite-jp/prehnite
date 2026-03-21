use prehnite_core::db::{acquire_err_handled, DBType};
use prehnite_core::util::alert::UnwrapOrErrorAlert;
use sqlx::pool::PoolConnection;
use sqlx::Sqlite;

pub mod query;

pub async fn acquire_book_with_alert() -> PoolConnection<Sqlite> {
    acquire_err_handled(DBType::PrehniteBook)
        .await
        .unwrap_or_alert()
}
