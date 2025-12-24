mod book_search_api;

use sqlx::FromRow;

pub type AppGlobalSetting = crate::db::schema::PrehniteBookSetting;

pub type AppGlobalDefaultTaskCategory = crate::db::schema::TaskCategory;

pub type AppGlobalDefaultTaskTemplate = crate::db::schema::TaskTemplate;

pub type AppGlobalDefaultPublisher = crate::db::schema::Publisher;

pub type AppGlobalDefaultBibliography = crate::db::schema::Bibliography;

#[derive(Default, Clone, FromRow)]
pub struct BookSearchApi {
    pub id: i64,
    pub name: String,
    pub detail: String,
    pub isbn_url: String,
    pub text_url: String,
    pub mapping_script: String,
    pub is_example: bool,
}
