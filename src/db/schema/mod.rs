#![allow(unused)]

pub mod custom_from_row;
mod load_impl;
mod prefixed_deserializer;

use chrono::{DateTime, Utc};
use sqlx::{Acquire, Connection, Database, FromRow, Row};

#[derive(Default, Clone, FromRow)]
pub struct BackgroundInfo {
    pub id: i64,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(skip)]
    pub references: Option<Vec<BackgroundReference>>,
}

#[derive(Default, Clone, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Clone)]
pub struct Bibliography {
    pub id: i64,
    pub isbn: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub detail: Option<String>,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub enum ItemType {
    Headline(Option<Headline>),
    Paragraph(Option<Paragraph>),
}

impl Default for ItemType {
    fn default() -> Self {
        Self::Headline(None)
    }
}

#[derive(Default, Clone)]
pub struct Item {
    pub id: i64,
    pub created_at: i64,
    pub item_type: ItemType,
    pub title: String,
    pub references: Option<Vec<ItemReference>>,
    pub tags: Option<Vec<Tag>>,
    pub background_info_list: Option<Vec<BackgroundInfo>>,
    pub tasks: Option<Vec<Task>>,
}

#[derive(Default, Clone, FromRow)]
pub struct Headline {
    pub id: i64,
    pub item_id: i64,
    pub parent_id: Option<i64>,
    pub headline_pos: Option<i64>,
    #[sqlx(skip)]
    pub children: Option<Vec<Option<i64>>>,
    #[sqlx(skip)]
    pub paragraph: Option<Vec<Paragraph>>,
}

#[derive(Default, Clone, FromRow)]
pub struct Draft {
    pub id: i64,
    pub paragraph_id: i64,
    pub draft_pos: Option<i64>,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default, Clone)]
pub struct Paragraph {
    pub id: i64,
    pub item_id: i64,
    pub headline: Headline,
    pub accepted_draft: Option<Draft>,
    pub paragraph_pos: Option<i64>,
    pub draft: Option<Vec<Draft>>,
    pub summary: Option<Vec<ParagraphSummary>>,
}

#[derive(Default, Clone, FromRow)]
pub struct ParagraphSummary {
    pub id: i64,
    pub title: String,
    pub detail: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default, Clone)]
pub struct BackgroundReference {
    pub id: i64,
    pub background_info_id: i64,
    pub bibliography: Bibliography,
    pub location: String,
}

#[derive(Default, Clone)]
pub struct ItemReference {
    pub id: i64,
    pub item_id: i64,
    pub bibliography: Bibliography,
    pub location: String,
}

#[derive(Default, Clone, FromRow)]
pub struct TaskCategory {
    pub id: i64,
    pub name: String,
    pub autocomplete_paragraph_link: bool,
}

#[derive(Default, Clone)]
pub struct TaskTemplate {
    pub id: i64,
    pub task_category: Option<TaskCategory>,
    pub title: String,
    pub detail: Option<String>,
}

#[derive(Default, Clone)]
pub struct Task {
    pub id: i64,
    pub task_category: Option<TaskCategory>,
    pub title: String,
    pub detail: Option<String>,
    pub is_finished: bool,
}

#[derive(Default, Clone)]
pub struct ParagraphLink {
    pub id: i64,
    pub from_paragraph: Paragraph,
    pub to_paragraph: Paragraph,
    pub task: Option<Task>,
    pub comment: Option<String>,
}
