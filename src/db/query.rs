use prehnite_core::db::schema::{Item, ItemType};
use prehnite_core::opt_unwrap_or_return;
use sqlx::SqliteConnection;
use std::collections::HashMap;

const FETCH_ROOT_HEADLINES_SQL: &str =
    include_str!("../../assets/query/fetch_root_headline_query.sql");

pub async fn fetch_root_headline_items(
    conn: &mut SqliteConnection,
    per_page: u8,
    page: u32,
) -> Result<HashMap<i64, Item>, sqlx::Error> {
    Ok(sqlx::query_as::<_, Item>(FETCH_ROOT_HEADLINES_SQL)
        .bind(per_page)
        .bind(page * per_page as u32)
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|v| (v.id, v))
        .collect())
}

const FETCH_ROOT_HEADLINE_RELATED_PARAGRAPH_SQL: &str =
    include_str!("../../assets/query/fetch_root_headline_related_paragraph.sql");

pub async fn fetch_root_headline_related_paragraph(
    conn: &mut SqliteConnection,
    headline_per_page: u8,
    headline_page: u32,
) -> Result<HashMap<i64, HashMap<i64, Item>>, sqlx::Error> {
    let mut result: HashMap<i64, HashMap<i64, Item>> = HashMap::new();
    sqlx::query_as::<_, Item>(FETCH_ROOT_HEADLINE_RELATED_PARAGRAPH_SQL)
        .bind(headline_per_page)
        .bind(headline_page * headline_per_page as u32)
        .fetch_all(conn)
        .await?
        .into_iter()
        .for_each(|v| {
            let p = match &v.item_type {
                ItemType::Headline(_) => return,
                ItemType::Paragraph(v) => {
                    opt_unwrap_or_return!(v, ())
                }
            };
            let headline_itm_id = p.headline.id;
            if result.contains_key(&headline_itm_id) {
                result.get_mut(&headline_itm_id).unwrap().insert(v.id, v);
            } else {
                let mut tmp = HashMap::new();
                tmp.insert(v.id, v);
                result.insert(headline_itm_id, tmp);
            }
        });
    Ok(result)
}
