#![allow(unused)]
#![doc = "データベースのスキーマ定義"]
#[cfg(feature = "backend")]
pub mod app_global;
#[cfg(feature = "backend")]
mod from_row;
#[cfg(feature = "backend")]
mod load;
#[cfg(feature = "backend")]
mod prefixed_deserializer;
#[cfg(test)]
mod tests;

use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use prehnite_core_proc_macros::{CreateRecord, DeleteRecord, ReadRecord, UpdateRecord};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, Database, FromRow, Row};
use std::fmt::{Display, Formatter};

/// 最大のプレースホルダ数。
/// sqlite 3.32.0 以降では32766が最大ですが、マージンを取って30000にしています。
pub const MAX_BIND_COUNT: usize = 30000;

#[derive(Default, Clone, Debug, FromRow, Eq, PartialEq, Deserialize, Serialize)]
/// 汎用のId取得クエリ用構造体
pub struct ReturningId {
    pub id: i64,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    derive_more::Eq,
    derive_more::PartialEq,
    ReadRecord,
    CreateRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
/// 背景情報
pub struct BackgroundInfo {
    pub id: i64,
    pub body: String,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub created_at: DateTime<Utc>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub updated_at: DateTime<Utc>,
    #[sqlx(skip)]
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub references: Option<Vec<BackgroundReference>>,
}

#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    Eq,
    PartialEq,
    ReadRecord,
    CreateRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "tags")]
/// タグ
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(
    Default,
    Clone,
    Debug,
    Eq,
    PartialEq,
    ReadRecord,
    CreateRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "publishers")]
/// 出版社
pub struct Publisher {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Eq,
    derive_more::PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(
    table_name = "bibliographies",
    view_name = "view_deserializable_bibliographies"
)]
/// 参考文献
pub struct Bibliography {
    pub id: i64,
    pub isbn: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub detail: Option<String>,
    #[prehnite_db(skip)]
    pub authors: Vec<BibliographyAuthor>,
    #[prehnite_db(use_id, name = "publisher_id")]
    pub publisher: Option<Publisher>,
    pub publication_date: Option<String>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub created_at: DateTime<Utc>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub updated_at: DateTime<Utc>,
    #[eq(skip)]
    #[prehnite_db(skip_update)]
    pub tmp_registration_id: Option<i64>,
}

#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "bibliography_authors")]
/// 参考文献の著者
pub struct BibliographyAuthor {
    pub id: i64,
    pub name: String,
    pub memo: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
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
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Eq,
    derive_more::PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "items", view_name = "view_deserializable_item")]
/// アイテム（見出しと段落のスーパータイプ）
pub struct Item {
    pub id: i64,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub created_at: i64,
    #[prehnite_db(use_string_from, skip_update)]
    pub item_type: ItemType,
    pub title: String,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub references: Option<Vec<ItemReference>>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub tags: Option<Vec<Tag>>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub background_info_list: Option<Vec<BackgroundInfo>>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub tasks: Option<Vec<Task>>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    derive_more::Eq,
    derive_more::PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "headlines")]
/// 見出しに固有の情報
pub struct Headline {
    pub id: i64,
    #[prehnite_db(skip_update)]
    pub item_id: i64,
    pub parent_id: Option<i64>,
    pub headline_pos: Option<i64>,
    #[sqlx(skip)]
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub paragraph: Option<Vec<Paragraph>>,
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
/// 特定の見出しに紐づいている子見出し
pub struct HeadlineChildren {
    pub parent: Headline,
    pub children: IndexMap<i64, Vec<Headline>>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    derive_more::Eq,
    derive_more::PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
/// 下書き
pub struct Draft {
    pub id: i64,
    pub paragraph_id: i64,
    pub draft_pos: Option<i64>,
    pub title: String,
    pub body: String,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub created_at: DateTime<Utc>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub updated_at: DateTime<Utc>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Eq,
    derive_more::PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(view_name = "view_deserializable_paragraph")]
/// 段落に固有の情報
pub struct Paragraph {
    pub id: i64,
    #[prehnite_db(skip_update)]
    pub item_id: i64,
    #[prehnite_db(use_id, name = "headline_id")]
    pub headline: Headline,
    #[prehnite_db(use_id, name = "accepted_draft_id")]
    pub accepted_draft: Option<Draft>,
    pub paragraph_pos: Option<i64>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub draft: Option<Vec<Draft>>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub summary: Option<Vec<ParagraphSummary>>,
}

//noinspection RsUnnecessaryQualifications: suppress false positive
//noinspection RsDerivableTraitMembers: suppress false positive
#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    derive_more::Eq,
    derive_more::PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "paragraph_summaries")]
/// 要約した段落の内容
pub struct ParagraphSummary {
    pub id: i64,
    pub paragraph_id: i64,
    pub title: String,
    pub detail: String,
    pub summary_pos: Option<i64>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub created_at: DateTime<Utc>,
    #[eq(skip)]
    #[prehnite_db(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Default,
    Clone,
    Debug,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(
    table_name = "background_references",
    view_name = "view_deserializable_background_reference"
)]
/// 背景情報と参考文献の関連付け
pub struct BackgroundReference {
    pub id: i64,
    pub background_info_id: i64,
    #[prehnite_db(use_id, name = "bibliography_id")]
    pub bibliography: Bibliography,
    pub location: String,
}

#[derive(
    Default,
    Clone,
    Debug,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(
    table_name = "item_references",
    view_name = "view_deserializable_item_reference"
)]
/// アイテムと参考文献の関連付け
pub struct ItemReference {
    pub id: i64,
    pub item_id: i64,
    #[prehnite_db(use_id, name = "bibliography_id")]
    pub bibliography: Bibliography,
    pub location: String,
}

#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "task_categories")]
/// タスクのカテゴリ
pub struct TaskCategory {
    pub id: i64,
    pub name: String,
    pub autocomplete_paragraph_link: bool,
}

#[derive(
    Default,
    Clone,
    Debug,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(
    table_name = "task_templates",
    view_name = "view_deserializable_task_template"
)]
/// タスクのテンプレート
pub struct TaskTemplate {
    pub id: i64,
    #[prehnite_db(use_id, name = "task_category_id")]
    pub task_category: Option<TaskCategory>,
    pub title: String,
    pub detail: Option<String>,
}

#[derive(
    Default,
    Clone,
    Debug,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "tasks", view_name = "view_deserializable_task")]
/// タスク
pub struct Task {
    pub id: i64,
    pub item_id: i64,
    #[prehnite_db(use_id, name = "task_category_id")]
    pub task_category: Option<TaskCategory>,
    pub title: String,
    pub detail: Option<String>,
    pub task_pos: Option<i64>,
    pub is_finished: bool,
}

#[derive(
    Default,
    Clone,
    Debug,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(view_name = "view_deserializable_paragraph_link")]
/// 段落間のリンク
pub struct ParagraphLink {
    pub id: i64,
    #[prehnite_db(use_id, name = "from_paragraph_id")]
    pub from_paragraph: Paragraph,
    #[prehnite_db(use_id, name = "to_paragraph_id")]
    pub to_paragraph: Paragraph,
    #[prehnite_db(use_id, name = "task_id")]
    pub task: Option<Task>,
    pub comment: Option<String>,
}

#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "settings")]
/// 設定値
pub struct Setting {
    pub id: i64,
    #[prehnite_db(skip_update)]
    pub setting_key: String,
    pub setting_value: Option<String>,
}

impl Setting {
    pub fn to_setting_row(self) -> easy_settings::SettingRow {
        easy_settings::SettingRow {
            setting_key: self.setting_key,
            value: self.setting_value,
        }
    }
}

#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
#[prehnite_db(table_name = "rel_bibliography_authors")]
/// 参考文献と著者の関連付け
pub struct RelBibliographyAuthor {
    pub id: i64,
    pub bibliography_id: i64,
    pub bibliography_author_id: i64,
}

#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
/// タグとアイテムの関連付け
pub struct RelTagAndItem {
    pub id: i64,
    pub item_id: i64,
    pub tag_id: i64,
}

#[derive(
    Default,
    Clone,
    Debug,
    FromRow,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
/// 背景情報とアイテムの関連付け
pub struct RelBackgroundAndItem {
    pub id: i64,
    pub item_id: i64,
    pub background_info_id: i64,
}

#[derive(
    Default,
    Debug,
    Clone,
    FromRow,
    Eq,
    PartialEq,
    CreateRecord,
    ReadRecord,
    UpdateRecord,
    DeleteRecord,
    Deserialize,
    Serialize,
)]
pub struct BookSearchApi {
    pub id: i64,
    pub name: String,
    pub detail: String,
    pub isbn_url: String,
    pub text_url: String,
    pub mapping_script: String,
    #[prehnite_db(skip)]
    pub is_example: bool,
}
