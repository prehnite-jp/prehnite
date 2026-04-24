use dioxus::prelude::{post, put};
use dioxus_i18n::prelude::i18n;
use prehnite_core::db::schema::{BookSearchApi, TaskCategory, TaskTemplate};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum OpenBookError {
    #[error("Failed to open book.")]
    FailedToOpenBook,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PrehniteBookFixtures {
    task_category: Vec<TaskCategory>,
    task_template: Vec<TaskTemplate>,
    book_search_api: Vec<BookSearchApi>,
}

#[cfg(feature = "desktop")]
impl Default for PrehniteBookFixtures {
    fn default() -> Self {
        let i18n = i18n();
        Self {
            task_category: vec![
                TaskCategory {
                    id: 1,
                    name: i18n.translate("task-category-foreshadowing"),
                    autocomplete_paragraph_link: true,
                },
                TaskCategory {
                    id: 2,
                    name: i18n.translate("task-category-unexplained"),
                    autocomplete_paragraph_link: true,
                },
            ],
            task_template: vec![
                TaskTemplate {
                    id: 1,
                    task_category: Some(TaskCategory {
                        id: 1,
                        ..Default::default()
                    }),
                    title: i18n.translate("task-template-recover"),
                    detail: Some(i18n.translate("task-template-recover-detail")),
                },
                TaskTemplate {
                    id: 2,
                    task_category: Some(TaskCategory {
                        id: 2,
                        ..Default::default()
                    }),
                    title: i18n.translate("task-template-will-explain"),
                    detail: Some(i18n.translate("task-template-will-explain-detail")),
                },
            ],
            book_search_api: vec![BookSearchApi {
                id: 0,
                name: i18n.translate("book-search-api-example-name"),
                detail: i18n.translate("book-search-api-example-detail"),
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
            }],
        }
    }
}

#[post("/api/book")]
/// PrehniteBookを開きます。存在しない場合は作成します。
pub async fn create_or_open_book(
    path: PathBuf,
    fixtures: PrehniteBookFixtures,
) -> anyhow::Result<()> {
    use crate::backend::db::{acquire_book, open_book_db_pool};
    use sqlx::Acquire;
    open_book_db_pool(path).await?;
    let mut conn = acquire_book()
        .await?
        .ok_or(OpenBookError::FailedToOpenBook)?;
    let mut tx = conn.begin().await?;
    let PrehniteBookFixtures {
        task_category,
        task_template,
        book_search_api,
    } = fixtures;

    for x in task_category {
        sqlx::query(
            "INSERT INTO task_categories(id, name, autocomplete_paragraph_link) VALUES (?,?,?)",
        )
        .bind(x.id)
        .bind(x.name)
        .bind(x.autocomplete_paragraph_link)
        .execute(&mut *tx)
        .await?;
    }

    TaskTemplate::register_many(task_template.as_slice(), &mut *tx, false).await?;
    BookSearchApi::register_many(book_search_api.as_slice(), &mut *tx, false).await?;

    tx.commit().await?;
    Ok(())
}

#[put("/api/book")]
/// 新しいPrehniteBookを開きます。既に存在する場合は上書きされます。
pub async fn open_new_book(path: PathBuf, fixtures: PrehniteBookFixtures) -> anyhow::Result<()> {
    tokio::fs::remove_file(&path).await?;
    create_or_open_book(path, fixtures).await
}
