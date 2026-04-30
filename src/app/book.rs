use crate::app::db::{acquire_book, open_book_db_pool};
use dioxus_i18n::t;
use prehnite_core::db::schema::{BookSearchApi, TaskCategory, TaskTemplate};
use serde::{Deserialize, Serialize};
use sqlx::Acquire;
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

impl Default for PrehniteBookFixtures {
    fn default() -> Self {
        Self {
            task_category: vec![
                TaskCategory {
                    id: 1,
                    name: t!("task_category_foreshadowing"),
                    autocomplete_paragraph_link: true,
                },
                TaskCategory {
                    id: 2,
                    name: t!("task_category_unexplained"),
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
                    title: t!("task_template_recover"),
                    detail: Some(t!("task_template_recover_detail")),
                },
                TaskTemplate {
                    id: 2,
                    task_category: Some(TaskCategory {
                        id: 2,
                        ..Default::default()
                    }),
                    title: t!("task_template_will_explain"),
                    detail: Some(t!("task_template_will_explain_detail")),
                },
            ],
            book_search_api: vec![BookSearchApi {
                id: 0,
                name: t!("book_search_api_example_name"),
                detail: t!("book_search_api_example_detail"),
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

/// PrehniteBookを開きます。存在しない場合は作成します。
pub async fn create_or_open_book(path: impl Into<PathBuf>) -> anyhow::Result<()> {
    open_book_db_pool(path).await?;
    let mut conn = acquire_book()
        .await?
        .ok_or(OpenBookError::FailedToOpenBook)?;
    let mut tx = conn.begin().await?;
    let PrehniteBookFixtures {
        task_category,
        task_template,
        book_search_api,
    } = Default::default();

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

/// 新しいPrehniteBookを開きます。既に存在する場合は上書きされます。
pub async fn open_new_book(path: impl Into<PathBuf>) -> anyhow::Result<()> {
    let path = path.into();
    if tokio::fs::try_exists(&path).await? {
        tokio::fs::remove_file(&path).await?;
    }
    create_or_open_book(path).await
}
