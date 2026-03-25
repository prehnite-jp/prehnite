#![allow(unused)]
#![doc = "データベースのスキーマ定義"]
pub mod app_global;
mod binder_helper;
mod crud;
mod from_row;
mod load;
mod prefixed_deserializer;
#[cfg(test)]
mod tests;

use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use sqlx::{Acquire, Database, FromRow, Row};
use std::fmt::{Display, Formatter};

/// 最大のプレースホルダ数。
/// sqlite 3.32.0 以降では32766が最大ですが、マージンを取って30000にしています。
pub const MAX_BIND_COUNT: usize = 30000;

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
/// 汎用のId取得クエリ用構造体
pub struct ReturningId {
    pub id: i64,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, FromRow, derive_more::Eq, derive_more::PartialEq)]
/// 背景情報
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
/// タグ
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
/// 出版社
pub struct Publisher {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, derive_more::Eq, derive_more::PartialEq)]
/// 参考文献
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
/// 参考文献の著者
pub struct BibliographyAuthor {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// アイテムの種類
pub enum ItemType {
    /// 見出し
    Headline(Option<Headline>),
    /// 段落
    Paragraph(Option<Paragraph>),
}

impl ItemType {
    /// 見出しを返します。見出しでないか、関連するオブジェクトが存在しない場合[`None`]を返します。
    pub fn headline_or_none(self) -> Option<Headline> {
        match self {
            ItemType::Headline(v) => v,
            ItemType::Paragraph(_) => None,
        }
    }

    /// 段落を返します。段落でないか、関連するオブジェクトが存在しない場合[`None`]を返します。
    pub fn paragraph_or_none(self) -> Option<Paragraph> {
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
/// アイテム（見出しと段落のスーパータイプ）
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
/// 見出しに固有の情報
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
/// 特定の見出しに紐づいている子見出し
pub struct HeadlineChildren {
    pub parent: Headline,
    pub children: IndexMap<i64, Vec<Headline>>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(Default, Clone, Debug, FromRow, derive_more::Eq, derive_more::PartialEq)]
/// 下書き
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
/// 段落に固有の情報
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
/// 要約した段落の内容
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
/// 背景情報と参考文献の関連付け
pub struct BackgroundReference {
    pub id: i64,
    pub background_info_id: i64,
    pub bibliography: Bibliography,
    pub location: String,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
/// アイテムと参考文献の関連付け
pub struct ItemReference {
    pub id: i64,
    pub item_id: i64,
    pub bibliography: Bibliography,
    pub location: String,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
/// タスクのカテゴリ
pub struct TaskCategory {
    pub id: i64,
    pub name: String,
    pub autocomplete_paragraph_link: bool,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
/// タスクのテンプレート
pub struct TaskTemplate {
    pub id: i64,
    pub task_category: Option<TaskCategory>,
    pub title: String,
    pub detail: Option<String>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
/// タスク
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
/// 段落間のリンク
pub struct ParagraphLink {
    pub id: i64,
    pub from_paragraph: Paragraph,
    pub to_paragraph: Paragraph,
    pub task: Option<Task>,
    pub comment: Option<String>,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
/// 設定値
pub struct Setting {
    pub id: i64,
    pub setting_key: String,
    pub setting_value: Option<String>,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
/// 参考文献と著者の関連付け
pub struct RelBibliographyAuthor {
    pub id: i64,
    pub bibliography_id: i64,
    pub bibliography_author_id: i64,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
/// タグとアイテムの関連付け
pub struct RelTagAndItem {
    pub id: i64,
    pub item_id: i64,
    pub tag_id: i64,
}

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq)]
/// 背景情報とアイテムの関連付け
pub struct RelBackgroundAndItem {
    pub id: i64,
    pub item_id: i64,
    pub background_info_id: i64,
}
