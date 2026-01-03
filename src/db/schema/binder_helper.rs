use crate::db::schema::{
    BackgroundInfo, BackgroundReference, Bibliography, BibliographyAuthor, Draft, Headline, Item,
    ItemReference, Paragraph, ParagraphLink, ParagraphSummary, PrehniteBookSetting, Publisher,
    RelBackgroundAndItem, RelBibliographyAuthor, RelTagAndItem, ReturningId, Tag, Task,
    TaskCategory, TaskTemplate,
};
use sqlx::query::{Query, QueryAs};
use sqlx::sqlite::SqliteArguments;
use sqlx::Sqlite;

pub fn placeholder_helper(placeholder: impl AsRef<str>, count: usize) -> String {
    vec![placeholder.as_ref(); count].join(",")
}

pub fn placeholder_in_clause<T>(id_list: &Vec<T>) -> String {
    placeholder_helper("?", id_list.len())
}

pub trait Binder {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>;

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>>;
}

impl Binder for BackgroundInfo {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(self.body)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.body)
    }
}

impl Binder for BackgroundReference {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.background_info_id)
            .bind(self.bibliography.id)
            .bind(self.location)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.background_info_id)
            .bind(self.bibliography.id)
            .bind(self.location)
    }
}
impl Binder for Bibliography {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.isbn)
            .bind(self.url)
            .bind(self.title)
            .bind(self.detail)
            .bind(self.publisher.map(|v| v.id))
            .bind(self.publication_date)
            .bind(self.tmp_registration_id.map(|v| v as i64))
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.isbn)
            .bind(self.url)
            .bind(self.title)
            .bind(self.detail)
            .bind(self.publisher.map(|v| v.id))
            .bind(self.publication_date)
    }
}
impl Binder for BibliographyAuthor {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(self.name).bind(self.memo)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.name).bind(self.memo)
    }
}
impl Binder for Draft {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.paragraph_id)
            .bind(self.draft_pos)
            .bind(self.title)
            .bind(self.body)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.paragraph_id)
            .bind(self.draft_pos)
            .bind(self.title)
            .bind(self.body)
    }
}
impl Binder for Headline {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.item_id)
            .bind(self.parent_id)
            .bind(self.headline_pos)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.parent_id).bind(self.headline_pos)
    }
}
impl Binder for Item {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(String::from(self.item_type)).bind(self.title)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.title)
    }
}
impl Binder for ItemReference {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.item_id)
            .bind(self.bibliography.id)
            .bind(self.location)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.item_id)
            .bind(self.bibliography.id)
            .bind(self.location)
    }
}
impl Binder for Paragraph {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.item_id)
            .bind(self.headline.id)
            .bind(self.paragraph_pos)
            .bind(self.accepted_draft.map(|v| v.id))
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.headline.id)
            .bind(self.paragraph_pos)
            .bind(self.accepted_draft.map(|v| v.id))
    }
}
impl Binder for ParagraphLink {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.from_paragraph.id)
            .bind(self.to_paragraph.id)
            .bind(self.task.map(|v| v.id))
            .bind(self.comment)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.from_paragraph.id)
            .bind(self.to_paragraph.id)
            .bind(self.task.map(|v| v.id))
            .bind(self.comment)
    }
}
impl Binder for ParagraphSummary {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.paragraph_id)
            .bind(self.title)
            .bind(self.detail)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.paragraph_id)
            .bind(self.title)
            .bind(self.detail)
    }
}
impl Binder for PrehniteBookSetting {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(self.setting_key).bind(self.setting_value)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.setting_value)
    }
}
impl Binder for Publisher {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(self.name).bind(self.memo)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.name).bind(self.memo)
    }
}
impl Binder for RelBackgroundAndItem {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(self.item_id).bind(self.background_info_id)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.item_id).bind(self.background_info_id)
    }
}
impl Binder for RelBibliographyAuthor {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.bibliography_id)
            .bind(self.bibliography_author_id)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.bibliography_id)
            .bind(self.bibliography_author_id)
    }
}
impl Binder for RelTagAndItem {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(self.item_id).bind(self.tag_id)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.item_id).bind(self.tag_id)
    }
}
impl Binder for Tag {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(self.name).bind(self.memo)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.name).bind(self.memo)
    }
}
impl Binder for Task {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.item_id)
            .bind(self.task_category.map(|v| v.id))
            .bind(self.title)
            .bind(self.detail)
            .bind(self.is_finished)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.item_id)
            .bind(self.task_category.map(|v| v.id))
            .bind(self.title)
            .bind(self.detail)
            .bind(self.is_finished)
    }
}
impl Binder for TaskCategory {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query.bind(self.name).bind(self.autocomplete_paragraph_link)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query.bind(self.name).bind(self.autocomplete_paragraph_link)
    }
}
impl Binder for TaskTemplate {
    fn register_bind_values<'a>(
        self,
        query: QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>>,
    ) -> QueryAs<'a, Sqlite, ReturningId, SqliteArguments<'a>> {
        query
            .bind(self.task_category.map(|v| v.id))
            .bind(self.title)
            .bind(self.detail)
    }

    fn update_bind_values<'a>(
        self,
        query: Query<'a, Sqlite, SqliteArguments<'a>>,
    ) -> Query<'a, Sqlite, SqliteArguments<'a>> {
        query
            .bind(self.task_category.map(|v| v.id))
            .bind(self.title)
            .bind(self.detail)
    }
}
