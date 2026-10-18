use crate::db::schema::{
    BackgroundReference, Bibliography, Draft, Headline, Item, ItemReference, ItemType, Paragraph,
    ParagraphLink, Publisher, Task, TaskCategory, TaskTemplate,
};
use crate::db::util::prefixer::Prefixer;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

pub struct PrefixedDeserializer;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid value")]
    ItemTypeDecodeError,
}

impl PrefixedDeserializer {
    pub fn publisher(
        row: &'_ SqliteRow,
        prefix_bibliography_publisher: impl AsRef<str>,
    ) -> Result<Publisher, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_bibliography_publisher.as_ref());
        Ok(Publisher {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            name: row.try_get(p.prefix("name"))?,
            memo: row.try_get(p.prefix("memo"))?,
        })
    }

    pub fn bibliography(
        row: &'_ SqliteRow,
        prefix_bibliography: impl AsRef<str>,
        prefix_bibliography_publisher: impl AsRef<str>,
    ) -> Result<Bibliography, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_bibliography.as_ref());
        Ok(Bibliography {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            isbn: row.try_get(p.prefix("isbn"))?,
            url: row.try_get(p.prefix("url"))?,
            title: row.try_get(p.prefix("title"))?,
            detail: row.try_get(p.prefix("detail"))?,
            authors: vec![],
            publisher: Self::publisher(row, prefix_bibliography_publisher).ok(),
            publication_date: row.try_get(p.prefix("publication_date"))?,
            created_at: row.try_get(p.prefix("created_at"))?,
            updated_at: row.try_get(p.prefix("updated_at"))?,
            tmp_registration_id: row.try_get::<Option<i64>, _>(p.prefix("tmp_registration_id"))?,
        })
    }

    pub fn item(
        row: &SqliteRow,
        prefix_item: impl AsRef<str>,
        prefix_paragraph: impl AsRef<str>,
        prefix_headline: impl AsRef<str>,
        prefix_draft: impl AsRef<str>,
    ) -> Result<Item, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_item.as_ref());
        let str_item_type: String = row.try_get(p.prefix("item_type"))?;
        // 分類する
        let no_value_item_type = if str_item_type == "headline" {
            ItemType::Headline(None)
        } else if str_item_type == "paragraph" {
            ItemType::Paragraph(None)
        } else {
            return Err(sqlx::Error::ColumnDecode {
                index: "item_type".to_string(),
                source: Box::new(Error::ItemTypeDecodeError),
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
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
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
    pub fn headline(row: &SqliteRow, prefix: impl AsRef<str>) -> Result<Headline, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix.as_ref());
        Ok(Headline {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            item_id: row.try_get(p.prefix("item_id"))?,
            parent_id: row.try_get(p.prefix("parent_id"))?,
            prev_headline_id: row.try_get(p.prefix("prev_headline_id"))?,
            next_headline_id: row.try_get(p.prefix("next_headline_id"))?,
            paragraph: None,
        })
    }

    // 下書きを処理。
    pub fn draft(row: &SqliteRow, prefix: impl AsRef<str>) -> Result<Draft, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix.as_ref());
        Ok(Draft {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            paragraph_id: row.try_get(p.prefix("paragraph_id"))?,
            prev_draft_id: row.try_get(p.prefix("prev_draft_id"))?,
            next_draft_id: row.try_get(p.prefix("next_draft_id"))?,
            title: row.try_get(p.prefix("title"))?,
            body: row.try_get(p.prefix("body"))?,
            created_at: row.try_get(p.prefix("created_at"))?,
            updated_at: row.try_get(p.prefix("updated_at"))?,
        })
    }

    // 段落を処理。
    pub fn paragraph(
        row: &SqliteRow,
        prefix_paragraph: impl AsRef<str>,
        prefix_headline: impl AsRef<str>,
        prefix_draft: impl AsRef<str>,
    ) -> Result<Paragraph, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_paragraph.as_ref());
        Ok(Paragraph {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            item_id: row.try_get(p.prefix("item_id"))?,
            headline: Self::headline(row, prefix_headline)?,
            accepted_draft: Self::draft(row, prefix_draft).ok(),
            prev_paragraph_id: row.try_get(p.prefix("prev_paragraph_id"))?,
            next_paragraph_id: row.try_get(p.prefix("next_paragraph_id"))?,
            draft: None,
            summary: None,
        })
    }

    pub fn background_reference(
        row: &'_ SqliteRow,
        prefix_reference: impl AsRef<str>,
        prefix_bibliography: impl AsRef<str>,
        prefix_bibliography_publisher: impl AsRef<str>,
    ) -> Result<BackgroundReference, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_reference.as_ref());
        Ok(BackgroundReference {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            background_info_id: row.try_get(p.prefix("background_info_id"))?,
            bibliography: Self::bibliography(
                row,
                prefix_bibliography,
                prefix_bibliography_publisher,
            )?,
            location: row.try_get(p.prefix("location"))?,
        })
    }

    pub fn item_reference(
        row: &'_ SqliteRow,
        prefix_reference: impl AsRef<str>,
        prefix_bibliography: impl AsRef<str>,
        prefix_bibliography_publisher: impl AsRef<str>,
    ) -> Result<ItemReference, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_reference.as_ref());
        Ok(ItemReference {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            item_id: row.try_get(p.prefix("item_id"))?,
            bibliography: Self::bibliography(
                row,
                prefix_bibliography,
                prefix_bibliography_publisher,
            )?,
            location: row.try_get(p.prefix("location"))?,
        })
    }

    pub fn category(
        row: &SqliteRow,
        prefix_task_category: impl AsRef<str>,
    ) -> Result<TaskCategory, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_task_category.as_ref());
        Ok(TaskCategory {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            name: row.try_get(p.prefix("name"))?,
            autocomplete_paragraph_link: row.try_get(p.prefix("autocomplete_paragraph_link"))?,
        })
    }

    pub fn task_template(
        row: &SqliteRow,
        prefix_task_template: impl AsRef<str>,
        prefix_task_category: impl AsRef<str>,
    ) -> Result<TaskTemplate, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_task_template.as_ref());
        Ok(TaskTemplate {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            task_category: Self::category(row, prefix_task_category).ok(),
            title: row.try_get(p.prefix("title"))?,
            detail: row.try_get(p.prefix("detail"))?,
        })
    }

    pub fn task(
        row: &SqliteRow,
        prefix_task: impl AsRef<str>,
        prefix_task_category: impl AsRef<str>,
    ) -> Result<Task, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_task.as_ref());
        Ok(Task {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
            item_id: row.try_get(p.prefix("item_id"))?,
            task_category: Self::category(row, prefix_task_category).ok(),
            title: row.try_get(p.prefix("title"))?,
            detail: row.try_get(p.prefix("detail"))?,
            prev_task_id: row.try_get(p.prefix("prev_task_id"))?,
            next_task_id: row.try_get(p.prefix("next_task_id"))?,
            is_finished: row.try_get(p.prefix("is_finished"))?,
        })
    }

    pub fn paragraph_link(
        row: &SqliteRow,
        prefix_paragraph_link: impl AsRef<str>,
        prefix_task: impl AsRef<str>,
        prefix_task_category: impl AsRef<str>,
        prefix_from_paragraph: impl AsRef<str>,
        prefix_from_headline: impl AsRef<str>,
        prefix_from_draft: impl AsRef<str>,
        prefix_to_paragraph: impl AsRef<str>,
        prefix_to_headline: impl AsRef<str>,
        prefix_to_draft: impl AsRef<str>,
    ) -> Result<ParagraphLink, sqlx::Error> {
        let mut p = Prefixer::with_prefix(prefix_paragraph_link.as_ref());
        Ok(ParagraphLink {
            id: row
                .try_get::<Option<i64>, _>(p.prefix("id"))?
                .ok_or(sqlx::Error::RowNotFound)?,
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
