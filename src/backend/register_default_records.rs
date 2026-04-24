use prehnite_core::db::schema::app_global::book_search_api::BookSearchApi;
use prehnite_core::db::schema::{TaskCategory, TaskTemplate};
use sqlx::{Acquire, SqliteConnection, SqliteTransaction};

/// タスクカテゴリの初期値を登録します。
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

/// タスクカテゴリとタスクテンプレートの初期値を登録します。
pub async fn register_default_data_task_category_and_templates(
    tx: &mut SqliteTransaction<'_>,
) -> Result<(), sqlx::Error> {
    register_default_data_task_categories(tx).await?;
    let values = vec![
        TaskTemplate {
            id: 1,
            task_category: Some(TaskCategory {
                id: 1,
                ..Default::default()
            }),
            title: i18n("task-template-recover"),
            detail: Some(i18n("task-template-recover-detail")),
        },
        TaskTemplate {
            id: 2,
            task_category: Some(TaskCategory {
                id: 2,
                ..Default::default()
            }),
            title: i18n("task-template-will-explain"),
            detail: Some(i18n("task-template-will-explain-detail")),
        },
    ];
    TaskTemplate::register_many(values.as_slice(), &mut *tx, false).await?;
    Ok(())
}

/// BookSearchAPIの設定例を登録します。
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
    .register(&mut *tx, false)
    .await?;
    Ok(())
}

/// すべての初期データを登録します。
pub async fn register_all_default_data(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;
    register_default_data_task_category_and_templates(&mut tx).await?;
    register_default_data_book_search_api(&mut tx).await?;
    tx.commit().await
}


#[cfg(test)]
mod tests {
    use crate::backend::register_default_records::register_all_default_data;
    use sqlx::SqlitePool;

    #[sqlx::test(migrator = "prehnite_core::db::migrate::app_global::MIGRATOR")]
    async fn valid_register_all_default_data(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        register_all_default_data(&mut *conn).await.unwrap();
    }
}
