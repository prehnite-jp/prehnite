use crate::db::schema::prefixed_deserializer::PrefixedDeserializer;
use crate::db::schema::{BackgroundReference, Bibliography, Item, ItemReference, Paragraph, ParagraphLink, Task, TaskTemplate};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, FromRow};

const PREFIX_PARGRAPH: &'static str = "p_";
const PREFIX_HEADLINE: &'static str = "h_";
const PREFIX_DRAFT: &'static str = "d_";
const PREFIX_TASK: &'static str = "t_";
const PREFIX_REFERENCE: &'static str = "r_";
const PREFIX_BIBLIOGRAPHY: &'static str = "b_";
const PREFIX_TASK_CATEGORY: &'static str = "tc_";
const PREFIX_PARAGRAPH_LINK: &'static str = "pl_";
const PREFIX_TASK_TEMPLATE: &'static str = "tt_";
const PREFIX_FROM_PARAGRAPH: &'static str = "from_p_";
const PREFIX_FROM_HEADLINE: &'static str = "from_h_";
const PREFIX_FROM_DRAFT: &'static str = "from_d_";
const PREFIX_TO_PARAGRAPH: &'static str = "to_p_";
const PREFIX_TO_HEADLINE: &'static str = "to_h_";
const PREFIX_TO_DRAFT: &'static str = "to_d_";

impl FromRow<'_, SqliteRow> for Bibliography {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        PrefixedDeserializer::bibliography(row, "")
    }
}

impl FromRow<'_, SqliteRow> for Item {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        PrefixedDeserializer::item(row, "", PREFIX_PARGRAPH, PREFIX_HEADLINE, PREFIX_DRAFT)
    }
}

impl FromRow<'_, SqliteRow> for Paragraph {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        PrefixedDeserializer::paragraph(row, "", PREFIX_HEADLINE, PREFIX_DRAFT)
    }
}

impl FromRow<'_, SqliteRow> for BackgroundReference {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        PrefixedDeserializer::background_reference(row, "", PREFIX_BIBLIOGRAPHY)
    }
}

impl FromRow<'_, SqliteRow> for ItemReference {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        PrefixedDeserializer::item_reference(row, "", PREFIX_BIBLIOGRAPHY)
    }
}

impl FromRow<'_, SqliteRow> for TaskTemplate {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        PrefixedDeserializer::task_template(row, "", PREFIX_TASK_CATEGORY)
    }
}

impl FromRow<'_, SqliteRow> for Task {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        PrefixedDeserializer::task(row, "", PREFIX_TASK_CATEGORY)
    }
}

impl FromRow<'_, SqliteRow> for ParagraphLink {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, Error> {
        PrefixedDeserializer::paragraph_link(
            row,
            "",
            PREFIX_TASK,
            PREFIX_TASK_CATEGORY,
            PREFIX_FROM_PARAGRAPH,
            PREFIX_FROM_HEADLINE,
            PREFIX_FROM_DRAFT,
            PREFIX_TO_PARAGRAPH,
            PREFIX_TO_HEADLINE,
            PREFIX_TO_DRAFT,
        )
    }
}
