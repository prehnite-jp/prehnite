use crate::db::error::ItemTypeDecodeError;
use crate::db::schema::{
    BackgroundReference, Bibliography, Draft, Headline, Item, ItemReference, ItemType, Paragraph,
    ParagraphLink, Task, TaskCategory, TaskTemplate,
};
use crate::util::Prefixer;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Row};

pub struct PrefixedDeserializer;

impl PrefixedDeserializer {
    pub fn bibliography(
        row: &'_ SqliteRow,
        prefix_bibliography: impl Into<String>,
    ) -> Result<Bibliography, Error> {
        let mut p = Prefixer::with_prefix(prefix_bibliography);
        Ok(Bibliography {
            id: row.try_get(p.prefix("id"))?,
            isbn: row.try_get(p.prefix("isbn"))?,
            url: row.try_get(p.prefix("url"))?,
            title: row.try_get(p.prefix("title"))?,
            detail: row.try_get(p.prefix("detail"))?,
            author: row.try_get(p.prefix("author"))?,
            created_at: row.try_get(p.prefix("created_at"))?,
            updated_at: row.try_get(p.prefix("updated_at"))?,
        })
    }

    pub fn item(
        row: &SqliteRow,
        prefix_item: impl Into<String>,
        prefix_paragraph: impl Into<String>,
        prefix_headline: impl Into<String>,
        prefix_draft: impl Into<String>,
    ) -> Result<Item, Error> {
        let mut p = Prefixer::with_prefix(prefix_item);
        let str_item_type: String = row.try_get(p.prefix("item_type"))?;
        // 分類する
        let no_value_item_type = if str_item_type == "headline" {
            ItemType::Headline(None)
        } else if str_item_type == "paragraph" {
            ItemType::Paragraph(None)
        } else {
            return Err(Error::ColumnDecode {
                index: "item_type".to_string(),
                source: Box::new(ItemTypeDecodeError("Failed to decode item_type.")),
            });
        };

        let item_type = match no_value_item_type {
            ItemType::Headline(_) => {
                ItemType::Headline(PrefixedDeserializer::headline(row, prefix_headline).ok())
            }
            ItemType::Paragraph(_) => ItemType::Paragraph(
                PrefixedDeserializer::paragraph(
                    row,
                    prefix_paragraph,
                    prefix_headline,
                    prefix_draft,
                )
                    .ok(),
            ),
        };

        Ok(Item {
            id: row.try_get(p.prefix("id"))?,
            created_at: row.try_get(p.prefix("created_at"))?,
            item_type,
            title: row.try_get(p.prefix("title"))?,
            references: None,
            tags: None,
            background_info_list: None,
            tasks: None,
        })
    }

    // 見出しを処理。
    pub fn headline(row: &SqliteRow, prefix: impl Into<String>) -> Result<Headline, Error> {
        let mut p = Prefixer::with_prefix(prefix);
        Ok(Headline {
            id: row.try_get(p.prefix("id"))?,
            item_id: row.try_get(p.prefix("item_id"))?,
            parent_id: row.try_get(p.prefix("parent_id"))?,
            headline_pos: row.try_get(p.prefix("headline_pos"))?,
            children: None,
            paragraph: None,
        })
    }

    // 下書きを処理。
    pub fn draft(row: &SqliteRow, prefix: impl Into<String>) -> Result<Draft, Error> {
        let mut p = Prefixer::with_prefix(prefix);
        Ok(Draft {
            id: row.try_get(p.prefix("id"))?,
            paragraph_id: row.try_get(p.prefix("paragraph_id"))?,
            draft_pos: row.try_get(p.prefix("draft_pos"))?,
            title: row.try_get(p.prefix("title"))?,
            body: row.try_get(p.prefix("body"))?,
            created_at: row.try_get(p.prefix("created_at"))?,
            updated_at: row.try_get(p.prefix("updated_at"))?,
        })
    }

    // 段落を処理。
    pub fn paragraph(
        row: &SqliteRow,
        prefix_paragraph: impl Into<String>,
        prefix_headline: impl Into<String>,
        prefix_draft: impl Into<String>,
    ) -> Result<Paragraph, Error> {
        let mut p = Prefixer::with_prefix(prefix_paragraph);
        Ok(Paragraph {
            id: row.try_get(p.prefix("id"))?,
            item_id: row.try_get(p.prefix("item_id"))?,
            headline: Self::headline(row, prefix_headline)?,
            accepted_draft: Self::draft(row, prefix_draft).ok(),
            paragraph_pos: row.try_get(p.prefix("paragraph_pos"))?,
            draft: None,
            summary: None,
        })
    }

    pub fn background_reference(
        row: &'_ SqliteRow,
        prefix_reference: impl Into<String>,
        prefix_bibliography: impl Into<String>,
    ) -> Result<BackgroundReference, Error> {
        let mut p = Prefixer::with_prefix(prefix_reference);
        Ok(BackgroundReference {
            id: row.try_get(p.prefix("id"))?,
            background_info_id: row.try_get(p.prefix("background_info_id"))?,
            bibliography: Self::bibliography(row, prefix_bibliography)?,
            location: row.try_get(p.prefix("location"))?,
        })
    }

    pub fn item_reference(
        row: &'_ SqliteRow,
        prefix_reference: impl Into<String>,
        prefix_bibliography: impl Into<String>,
    ) -> Result<ItemReference, Error> {
        let mut p = Prefixer::with_prefix(prefix_reference);
        Ok(ItemReference {
            id: row.try_get(p.prefix("id"))?,
            item_id: row.try_get(p.prefix("item_id"))?,
            bibliography: Self::bibliography(row, prefix_bibliography)?,
            location: row.try_get(p.prefix("location"))?,
        })
    }

    pub fn category(
        row: &SqliteRow,
        prefix_task_category: impl Into<String>,
    ) -> Result<TaskCategory, Error> {
        let mut p = Prefixer::with_prefix(prefix_task_category);
        Ok(TaskCategory {
            id: row.try_get(p.prefix("id"))?,
            name: row.try_get(p.prefix("name"))?,
            autocomplete_paragraph_link: row.try_get(p.prefix("autocomplete_paragraph_link"))?,
        })
    }

    pub fn task_template(
        row: &SqliteRow,
        prefix_task_template: impl Into<String>,
        prefix_task_category: impl Into<String>,
    ) -> Result<TaskTemplate, Error> {
        let mut p = Prefixer::with_prefix(prefix_task_template);
        Ok(TaskTemplate {
            id: row.try_get(p.prefix("id"))?,
            task_category: Self::category(row, prefix_task_category).ok(),
            title: row.try_get(p.prefix("title"))?,
            detail: row.try_get(p.prefix("detail"))?,
        })
    }

    pub fn task(
        row: &SqliteRow,
        prefix_task: impl Into<String>,
        prefix_task_category: impl Into<String>,
    ) -> Result<Task, Error> {
        let mut p = Prefixer::with_prefix(prefix_task);
        Ok(Task {
            id: row.try_get(p.prefix("id"))?,
            task_category: Self::category(row, prefix_task_category).ok(),
            title: row.try_get(p.prefix("title"))?,
            detail: row.try_get(p.prefix("detail"))?,
            is_finished: row.try_get(p.prefix("is_finished"))?,
        })
    }

    pub fn paragraph_link(
        row: &SqliteRow,
        prefix_paragraph_link: impl Into<String>,
        prefix_task: impl Into<String>,
        prefix_task_category: impl Into<String>,
        prefix_from_paragraph: impl Into<String>,
        prefix_from_headline: impl Into<String>,
        prefix_from_draft: impl Into<String>,
        prefix_to_paragraph: impl Into<String>,
        prefix_to_headline: impl Into<String>,
        prefix_to_draft: impl Into<String>,
    ) -> Result<ParagraphLink, Error> {
        let mut p = Prefixer::with_prefix(prefix_paragraph_link);
        Ok(ParagraphLink {
            id: row.try_get(p.prefix("id"))?,
            from_paragraph: PrefixedDeserializer::paragraph(
                row,
                prefix_from_paragraph,
                prefix_from_headline,
                prefix_from_draft,
            )?,
            to_paragraph: PrefixedDeserializer::paragraph(
                row,
                prefix_to_paragraph,
                prefix_to_headline,
                prefix_to_draft,
            )?,
            task: PrefixedDeserializer::task(row, prefix_task, prefix_task_category).ok(),
            comment: row.try_get(p.prefix("comment"))?,
        })
    }
}
