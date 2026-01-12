use prehnite_core::db::schema::Item;
use sqlx::SqliteConnection;

const FETCH_ROOT_HEADLINES: &str = include_str!("../../assets/query/fetch_root_headline_query.sql");

pub async fn fetch_root_headline_items(
    conn: &mut SqliteConnection,
    per_page: u8,
    page: u32,
) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as(FETCH_ROOT_HEADLINES)
        .bind(per_page)
        .bind(page * per_page as u32)
        .fetch_all(conn)
        .await
}
