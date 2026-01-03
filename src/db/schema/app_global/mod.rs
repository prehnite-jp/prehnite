pub mod book_search_api;
pub mod book_search_result;

use sqlx::FromRow;

pub type AppGlobalSetting = crate::db::schema::PrehniteBookSetting;

pub type AppGlobalDefaultTaskCategory = crate::db::schema::TaskCategory;

pub type AppGlobalDefaultTaskTemplate = crate::db::schema::TaskTemplate;

pub type AppGlobalDefaultTag = crate::db::schema::Tag;

pub type AppGlobalDefaultPublisher = crate::db::schema::Publisher;

pub type AppGlobalDefaultBibliography = crate::db::schema::Bibliography;
