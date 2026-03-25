#![allow(unused)]

pub mod app_global;
mod binder_helper;
pub mod crud;
pub mod from_row;
pub mod load;
mod prefixed_deserializer;
#[cfg(test)]
mod tests;

use chrono::{DateTime, Utc};
use sqlx::{Acquire, Database, FromRow, Row};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// 最大のプレースホルダ数。
/// sqlite 3.32.0 以降では32766が最大ですが、マージンを取って30000にしています。
pub const MAX_BIND_COUNT: usize = 30000;

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
pub struct ReturningId {
    pub id: i64,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, FromRow, derive_more::Eq, derive_more::PartialEq)]
pub struct BackgroundInfo {
    pub id: i64,
    pub body: String,
    #[eq(skip)]
    pub created_at: DateTime<Utc>,
    #[eq(skip)]
    pub updated_at: DateTime<Utc>,
    #[sqlx(skip)]
    #[eq(skip)]
    pub references: Option<Vec<BackgroundReference>>,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Publisher {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, derive_more::Eq, derive_more::PartialEq)]
pub struct Bibliography {
    pub id: i64,
    pub isbn: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub detail: Option<String>,
    pub authors: Vec<BibliographyAuthor>,
    pub publisher: Option<Publisher>,
    pub publication_date: Option<String>,
    #[eq(skip)]
    pub created_at: DateTime<Utc>,
    #[eq(skip)]
    pub updated_at: DateTime<Utc>,
    #[eq(skip)]
    pub tmp_registration_id: Option<usize>,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
pub struct BibliographyAuthor {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ItemType {
    Headline(Option<Headline>),
    Paragraph(Option<Paragraph>),
}

impl ItemType {
    pub fn headline_unwrap_or_default(self) -> Option<Headline> {
        match self {
            ItemType::Headline(v) => v,
            ItemType::Paragraph(_) => None,
        }
    }

    pub fn paragraph_unwrap_or_default(self) -> Option<Paragraph> {
        match self {
            ItemType::Headline(_) => None,
            ItemType::Paragraph(v) => v,
        }
    }
}

impl Default for ItemType {
    fn default() -> Self {
        Self::Headline(None)
    }
}

impl AsRef<str> for ItemType {
    fn as_ref(&self) -> &str {
        match self {
            ItemType::Headline(_) => "headline",
            ItemType::Paragraph(_) => "paragraph",
        }
    }
}

impl From<ItemType> for String {
    fn from(value: ItemType) -> Self {
        value.to_string()
    }
}

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, derive_more::Eq, derive_more::PartialEq)]
pub struct Item {
    pub id: i64,
    #[eq(skip)]
    pub created_at: i64,
    pub item_type: ItemType,
    pub title: String,
    #[eq(skip)]
    pub references: Option<Vec<ItemReference>>,
    #[eq(skip)]
    pub tags: Option<Vec<Tag>>,
    #[eq(skip)]
    pub background_info_list: Option<Vec<BackgroundInfo>>,
    #[eq(skip)]
    pub tasks: Option<Vec<Task>>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, FromRow, derive_more::Eq, derive_more::PartialEq)]
pub struct Headline {
    pub id: i64,
    pub item_id: i64,
    pub parent_id: Option<i64>,
    pub headline_pos: Option<i64>,
    #[sqlx(skip)]
    #[eq(skip)]
    pub paragraph: Option<Vec<Paragraph>>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct HeadlineChildren {
    pub parent: Headline,
    pub children: IndexMap<i64, Vec<Headline>>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, FromRow, derive_more::Eq, derive_more::PartialEq)]
pub struct Draft {
    pub id: i64,
    pub paragraph_id: i64,
    pub draft_pos: Option<i64>,
    pub title: String,
    pub body: String,
    #[eq(skip)]
    pub created_at: DateTime<Utc>,
    #[eq(skip)]
    pub updated_at: DateTime<Utc>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, derive_more::Eq, derive_more::PartialEq)]
pub struct Paragraph {
    pub id: i64,
    pub item_id: i64,
    pub headline: Headline,
    pub accepted_draft: Option<Draft>,
    pub paragraph_pos: Option<i64>,
    #[eq(skip)]
    pub draft: Option<Vec<Draft>>,
    #[eq(skip)]
    pub summary: Option<Vec<ParagraphSummary>>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, FromRow, derive_more::Eq, derive_more::PartialEq)]
pub struct ParagraphSummary {
    pub id: i64,
    pub paragraph_id: i64,
    pub title: String,
    pub detail: String,
    pub summary_pos: Option<i64>,
    #[eq(skip)]
    pub created_at: DateTime<Utc>,
    #[eq(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct BackgroundReference {
    pub id: i64,
    pub background_info_id: i64,
    pub bibliography: Bibliography,
    pub location: String,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ItemReference {
    pub id: i64,
    pub item_id: i64,
    pub bibliography: Bibliography,
    pub location: String,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
pub struct TaskCategory {
    pub id: i64,
    pub name: String,
    pub autocomplete_paragraph_link: bool,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct TaskTemplate {
    pub id: i64,
    pub task_category: Option<TaskCategory>,
    pub title: String,
    pub detail: Option<String>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Task {
    pub id: i64,
    pub item_id: i64,
    pub task_category: Option<TaskCategory>,
    pub title: String,
    pub detail: Option<String>,
    pub task_pos: Option<i64>,
    pub is_finished: bool,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ParagraphLink {
    pub id: i64,
    pub from_paragraph: Paragraph,
    pub to_paragraph: Paragraph,
    pub task: Option<Task>,
    pub comment: Option<String>,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
pub struct Setting {
    pub id: i64,
    pub setting_key: String,
    pub setting_value: Option<String>,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
pub struct RelBibliographyAuthor {
    pub id: i64,
    pub bibliography_id: i64,
    pub bibliography_author_id: i64,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
pub struct RelTagAndItem {
    pub id: i64,
    pub item_id: i64,
    pub tag_id: i64,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
pub struct RelBackgroundAndItem {
    pub id: i64,
    pub item_id: i64,
    pub background_info_id: i64,
}
