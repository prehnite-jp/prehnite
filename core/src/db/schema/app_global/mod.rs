pub mod book_search_api;
pub mod book_search_result;

use crate::db::schema::app_global::book_search_api::BookSearchApi;
use crate::i18n::i18n;
use sqlx::{Acquire, FromRow, SqliteConnection, SqliteTransaction};

pub type AppGlobalDefaultTaskCategory = crate::db::schema::TaskCategory;

pub type AppGlobalDefaultTaskTemplate = crate::db::schema::TaskTemplate;

pub type AppGlobalDefaultTag = crate::db::schema::Tag;

pub type AppGlobalDefaultPublisher = crate::db::schema::Publisher;

pub type AppGlobalDefaultBibliography = crate::db::schema::Bibliography;

pub type AppGlobalDefaultBibliographyAuthor = crate::db::schema::BibliographyAuthor;

pub type AppGlobalDefaultRelBibliographyAuthor = crate::db::schema::RelBibliographyAuthor;

async fn register_default_data_task_categories(
    tx: &mut SqliteTransaction<'_>,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO task_categories(id, name, autocomplete_paragraph_link) VALUES (?,?,?), (?,?,?)")
        .bind(1)
        .bind(i18n("task-category-foreshadowing"))
        .bind(true)
        .bind(2)
        .bind(i18n("task-category-unexplained"))
        .bind(true)
        .execute(&mut **tx).await?;
    Ok(())
}

pub async fn register_default_data_task_category_and_templates(
    tx: &mut SqliteTransaction<'_>,
) -> Result<(), sqlx::Error> {
    register_default_data_task_categories(tx).await?;
    let values = vec![
        AppGlobalDefaultTaskTemplate {
            id: 1,
            task_category: Some(AppGlobalDefaultTaskCategory {
                id: 1,
                ..Default::default()
            }),
            title: i18n("task-template-recover"),
            detail: Some(i18n("task-template-recover-detail")),
        },
        AppGlobalDefaultTaskTemplate {
            id: 2,
            task_category: Some(AppGlobalDefaultTaskCategory {
                id: 2,
                ..Default::default()
            }),
            title: i18n("task-template-will-explain"),
            detail: Some(i18n("task-template-will-explain-detail")),
        },
    ];
    AppGlobalDefaultTaskTemplate::register_vec_tx(values.as_slice(), tx, false).await?;
    Ok(())
}

pub async fn register_default_data_book_search_api(
    tx: &mut SqliteTransaction<'_>,
) -> Result<(), sqlx::Error> {
    BookSearchApi {
        id: 0,
        name: i18n("book-search-api-example-name"),
        detail: i18n("book-search-api-example-detail"),
        isbn_url: "https://example.com/api/book?isbn=<isbn>".to_string(),
        text_url: "https://example.com/api/book?search=<text>".to_string(),
        mapping_script: r#"fn mapper(isbn, search_text, response){
    let x = [];
    for result in response.result {
        x += new_res(
            result.isbn, // isbn
            "", // url
            result.title, // title
            result.detail, // detail
            result.authors, // authors
            (), // publisher (Option::None)
            result.publication_date, // publication date
        )
    }
    x
}"#
        .to_string(),
        is_example: true,
    }
    .register_tx(tx, false)
    .await?;
    Ok(())
}

pub async fn register_all_default_data(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;
    register_default_data_task_category_and_templates(&mut tx).await?;
    register_default_data_book_search_api(&mut tx).await?;
    tx.commit().await
}

#[cfg(test)]
mod tests {
    use crate::db::schema::app_global::register_all_default_data;
    use crate::i18n::initialize_i18n_from_db;
    use sqlx::SqlitePool;

    #[sqlx::test(migrator = "crate::db::migrate::app_global::MIGRATOR")]
    async fn valid_register_all_default_data(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        initialize_i18n_from_db(&mut conn).await;
        register_all_default_data(&mut conn).await.unwrap();
    }
}
