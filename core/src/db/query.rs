use crate::db::schema::{
    BackgroundInfo, BackgroundReference, BibliographyAuthor, Draft, Headline, HeadlineChildren,
    ItemReference, Paragraph, ParagraphSummary, Setting, Tag, Task,
};
use crate::settings::SettingKey;
use crate::{on_error_logging, opt_unwrap_or_continue, opt_unwrap_or_return, to_hash_map_key_id};
use sqlx::SqliteConnection;
use std::collections::HashMap;
use tracing::error;

const UPDATE_SETTING_SQL: &str = include_str!("../../assets/query/update_settings.sql");
#[tracing::instrument]
pub async fn update_setting(
    conn: &mut SqliteConnection,
    setting_key: SettingKey,
    setting_value: Option<String>,
) -> Result<(), sqlx::Error> {
    let v = sqlx::query(UPDATE_SETTING_SQL)
        .bind(setting_key.to_string())
        .bind(setting_value)
        .execute(conn)
        .await;
    on_error_logging!(v);
    Ok(())
}

const FETCH_SETTING_SQL: &str = include_str!("../../assets/query/fetch_settings.sql");
#[tracing::instrument]
pub async fn fetch_setting(
    conn: &mut SqliteConnection,
    setting_key: SettingKey,
) -> Result<Option<Setting>, sqlx::Error> {
    let v = sqlx::query_as::<_, Setting>(FETCH_SETTING_SQL)
        .bind(setting_key.to_string())
        .fetch_optional(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_BACKGROUND_INFO_FROM_ITEM_ID_SQL: &str =
    include_str!("../../assets/query/fetch_background_info_from_item_id.sql");
#[tracing::instrument]
pub async fn fetch_background_info_from_item_id(
    conn: &mut SqliteConnection,
    item_id: i64,
) -> Result<Vec<BackgroundInfo>, sqlx::Error> {
    let v = sqlx::query_as::<_, BackgroundInfo>(FETCH_BACKGROUND_INFO_FROM_ITEM_ID_SQL)
        .bind(item_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_HEADLINE_CHILDREN_RECURSE_SQL: &str =
    include_str!("../../assets/query/fetch_headline_children_recurse.sql");
#[tracing::instrument]
pub async fn fetch_headline_children_recurse(
    conn: &mut SqliteConnection,
    headline_id: i64,
) -> Result<Option<HeadlineChildren>, sqlx::Error> {
    let query_result = sqlx::query_as::<_, Headline>(FETCH_HEADLINE_CHILDREN_RECURSE_SQL)
        .bind(headline_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(query_result);
    let headlines: HashMap<i64, Headline> = to_hash_map_key_id!(query_result?);
    let parent = opt_unwrap_or_return!(headlines.get(&headline_id).cloned(), Ok(None));
    let mut children: HashMap<i64, Vec<Headline>> = HashMap::new();
    for i in headlines.keys() {
        let headline = opt_unwrap_or_continue!(headlines.get(i));
        match children.get_mut(&opt_unwrap_or_continue!(headline.parent_id)) {
            None => {
                children.insert(headline.parent_id.unwrap(), vec![headline.clone()]);
            }
            Some(v) => v.push(headline.clone()),
        }
    }
    Ok(Some(HeadlineChildren { parent, children }))
}

const FETCH_BACKGROUND_REFERENCES_SQL: &str =
    include_str!("../../assets/query/fetch_background_references.sql");
#[tracing::instrument]
pub async fn fetch_background_references(
    conn: &mut SqliteConnection,
    background_info_id: i64,
) -> Result<Vec<BackgroundReference>, sqlx::Error> {
    let v = sqlx::query_as::<_, BackgroundReference>(FETCH_BACKGROUND_REFERENCES_SQL)
        .bind(background_info_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_BIBLIOGRAPHY_AUTHORS_SQL: &str =
    include_str!("../../assets/query/fetch_bibliography_authors.sql");
#[tracing::instrument]
pub async fn fetch_bibliography_authors(
    conn: &mut SqliteConnection,
    bibliography_id: i64,
) -> Result<Vec<BibliographyAuthor>, sqlx::Error> {
    let v = sqlx::query_as::<_, BibliographyAuthor>(FETCH_BIBLIOGRAPHY_AUTHORS_SQL)
        .bind(bibliography_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_ITEM_REFERENCES_SQL: &str =
    include_str!("../../assets/query/fetch_item_references.sql");
#[tracing::instrument]
pub async fn fetch_item_references(
    conn: &mut SqliteConnection,
    item_id: i64,
) -> Result<Vec<ItemReference>, sqlx::Error> {
    let v = sqlx::query_as::<_, ItemReference>(FETCH_ITEM_REFERENCES_SQL)
        .bind(item_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_ITEM_RELATED_TAGS_SQL: &str =
    include_str!("../../assets/query/fetch_item_related_tags.sql");
#[tracing::instrument]
pub async fn fetch_item_related_tags(
    conn: &mut SqliteConnection,
    item_id: i64,
) -> Result<Vec<Tag>, sqlx::Error> {
    let v = sqlx::query_as::<_, Tag>(FETCH_ITEM_RELATED_TAGS_SQL)
        .bind(item_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_ITEM_RELATED_TASKS_SQL: &str =
    include_str!("../../assets/query/fetch_item_related_tasks.sql");
#[tracing::instrument]
pub async fn fetch_item_related_tasks(
    conn: &mut SqliteConnection,
    item_id: i64,
) -> Result<Vec<Task>, sqlx::Error> {
    let v = sqlx::query_as::<_, Task>(FETCH_ITEM_RELATED_TASKS_SQL)
        .bind(item_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_HEADLINE_RELATED_PARAGRAPH_SQL: &str =
    include_str!("../../assets/query/fetch_headline_related_paragraph.sql");
#[tracing::instrument]
pub async fn fetch_headline_related_paragraph(
    conn: &mut SqliteConnection,
    headline_id: i64,
) -> Result<Vec<Paragraph>, sqlx::Error> {
    let v = sqlx::query_as::<_, Paragraph>(FETCH_HEADLINE_RELATED_PARAGRAPH_SQL)
        .bind(headline_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_PARAGRAPH_RELATED_SUMMARIES_SQL: &str =
    include_str!("../../assets/query/fetch_paragraph_related_summaries.sql");
#[tracing::instrument]
pub async fn fetch_paragraph_related_summaries(
    conn: &mut SqliteConnection,
    paragraph_id: i64,
) -> Result<Vec<ParagraphSummary>, sqlx::Error> {
    let v = sqlx::query_as::<_, ParagraphSummary>(FETCH_PARAGRAPH_RELATED_SUMMARIES_SQL)
        .bind(paragraph_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}

const FETCH_PARAGRAPH_RELATED_DRAFT_SQL: &str =
    include_str!("../../assets/query/fetch_paragraph_related_draft.sql");
#[tracing::instrument]
pub async fn fetch_paragraph_related_draft(
    conn: &mut SqliteConnection,
    paragraph_id: i64,
) -> Result<Vec<Draft>, sqlx::Error> {
    let v = sqlx::query_as::<_, Draft>(FETCH_PARAGRAPH_RELATED_DRAFT_SQL)
        .bind(paragraph_id)
        .fetch_all(conn)
        .await;
    on_error_logging!(v);
    v
}
