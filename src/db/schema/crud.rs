use crate::db::schema::app_global::book_search_api::BookSearchApi;
use crate::db::schema::binder_helper::{Binder, placeholder_helper, placeholder_in_clause};
use crate::db::schema::{
    BackgroundInfo, BackgroundReference, Bibliography, BibliographyAuthor, Draft, Headline, Item,
    ItemReference, Paragraph, ParagraphLink, ParagraphSummary, Publisher, RelBackgroundAndItem,
    RelBibliographyAuthor, RelTagAndItem, ReturningId, Setting, Tag, Task, TaskCategory,
    TaskTemplate,
};
use sqlx::{Acquire, Error, SqliteConnection, SqliteTransaction};

const MAX_BIND_COUNT: usize = 30000; // sqlite 3.32.0 以降では32766が最大だが、マージンを取って30000

fn first_or_row_not_found<T>(values: &Vec<T>) -> Result<T, Error>
where
    T: Clone,
{
    if values.is_empty() {
        Err(Error::RowNotFound)
    } else {
        Ok(values[0].clone())
    }
}

macro_rules! allow_r {
    ($(($x: ty, $view_name:expr)),*) => {
        $(impl $x {
            pub async fn select_all(conn: &mut SqliteConnection) ->  Result<Vec<Self>, Error> {
                let mut tx = conn.begin().await?;
                let result = Self::select_all_tx(&mut tx).await?;
                tx.commit().await?;
                Ok(result)
            }

            pub async fn select_all_tx(tx: &mut SqliteTransaction<'_>) ->  Result<Vec<Self>, Error> {
                sqlx::query_as(concat!("SELECT * FROM ", $view_name))
                .fetch_all(&mut **tx).await
            }
        })*
    };
}

macro_rules! allow_c {
    ($(($x: ty, $table_name:expr, $view_name:expr, $register_columns:expr, $place_holder_count:expr)),*) => {
        $(impl $x {
            pub async fn register_optional(val: Option<Self>, conn: &mut SqliteConnection, is_on_conflict_do_nothing: bool) -> Result<Option<Self>, Error> {
                Ok(match val {
                    Some(v) => Some(v.register(conn, is_on_conflict_do_nothing).await?),
                    None => None
                })
            }

            pub async fn register_optional_tx(val: Option<Self>, tx: &mut SqliteTransaction<'_>, is_on_conflict_do_nothing: bool) -> Result<Option<Self>, Error> {
                Ok(match val{
                    Some(v) => Some(v.register_tx(tx, is_on_conflict_do_nothing).await?),
                    None => None
                })
            }

            pub async fn register(&self, conn: &mut SqliteConnection, is_on_conflict_do_nothing: bool) -> Result<Self, Error> {
                first_or_row_not_found(&Self::register_vec(&vec![self.clone()], conn, is_on_conflict_do_nothing).await?)
            }

            pub async fn register_tx(&self, tx: &mut SqliteTransaction<'_>, is_on_conflict_do_nothing: bool) -> Result<Self, Error> {
                first_or_row_not_found(&Self::register_vec_tx(&vec![self.clone()], tx, is_on_conflict_do_nothing).await?)
            }

            pub async fn register_vec(
                values: &[Self],
                conn: &mut SqliteConnection,
                is_on_conflict_do_nothing: bool
            ) -> Result<Vec<Self>, Error> {
                let mut tx = conn.begin().await?;
                let result = Self::register_vec_tx(values, &mut tx, is_on_conflict_do_nothing).await?;
                tx.commit().await?;
                Ok(result)
            }

            pub async fn register_vec_tx(values: &[Self], tx: &mut SqliteTransaction<'_>, is_on_conflict_do_nothing: bool) -> Result<Vec<Self>, Error> {
                if values.is_empty() {
                    return Ok(vec![]);
                }
                let mut v = Vec::new();
                for i in values.chunks(MAX_BIND_COUNT / $place_holder_count) {
                    let sql = format!(
                        concat!(
                            "INSERT INTO ",
                            $table_name,
                            "(",
                            $register_columns,
                            ") VALUES {} {} RETURNING id"
                        ),
                        placeholder_helper(format!("({})", placeholder_helper("?", $place_holder_count)), i.len()),
                        if is_on_conflict_do_nothing { "ON CONFLICT DO NOTHING" } else { "" }
                    );
                    let mut query = sqlx::query_as(sql.as_str());
                    for i in i {
                        query = Binder::register_bind_values(i.clone(), query)
                    }
                    let id_list: Vec<ReturningId> = query.fetch_all(&mut **tx).await?;
                    let sql = format!(
                        concat!("SELECT * FROM ", $view_name, " WHERE id IN ({})"),
                        placeholder_in_clause(id_list.as_ref())
                    );
                    let mut query = sqlx::query_as(sql.as_str());
                    for i in id_list {
                        query = query.bind(i.id);
                    }
                    v.extend(query.fetch_all(&mut **tx).await?);
                }
                Ok(v)
            }
        })*
    };
}
macro_rules! allow_u {
    ($(($x: ty, $table_name:expr, $update_set_clause:expr)),*) => {
    $(impl $x {
        pub async fn update(&self, conn: &mut SqliteConnection) -> Result<(), Error> {
            let mut tx = conn.begin().await?;
            self.update_with_tx(&mut tx).await?;
            tx.commit().await
        }

        pub async fn update_with_tx(&self, tx: &mut SqliteTransaction<'_>) -> Result<(), Error> {
            let mut query = sqlx::query(concat!(
                "UPDATE ",
                $table_name,
                " SET ",
                $update_set_clause,
                " WHERE id=?"
            ));
            self
                .clone()
                .update_bind_values(query)
                .bind(self.id)
                .execute(&mut **tx)
                .await?;
            return Ok(());
        }
    })*
    };
}
macro_rules! allow_d {
    ($(($x:ty, $table_name:expr)),*) => {
        $(
        impl $x {
            pub async fn delete(self, conn: &mut SqliteConnection) -> Result<(), Error>
            where
                Self: Sized,
            {
                let mut tx = conn.begin().await?;
                self.delete_with_tx(&mut tx).await?;
                tx.commit().await
            }

            pub async fn delete_with_tx(self, tx: &mut SqliteTransaction<'_>) -> Result<(), Error> {
                sqlx::query(concat!("DELETE FROM ", $table_name, " WHERE id = ?"))
                    .bind(self.id)
                    .execute(&mut **tx)
                    .await?;
                Ok(())
            }
        }
        )*
    };
}
macro_rules! allow_crud {
    ($(($x: ty, $table_name:expr, $view_name:expr, $register_columns:expr, $place_holder:expr, $update_set_clause:expr)),*) => {
        $(
            allow_c!(($x, $table_name, $view_name, $register_columns, $place_holder));
            allow_r!(($x, $view_name));
            allow_u!(($x, $table_name, $update_set_clause));
            allow_d!(($x, $table_name));
        )*
    };
}
macro_rules! allow_cru {
    ($(($x: ty, $table_name:expr, $view_name:expr, $register_columns:expr, $place_holder:expr, $update_set_clause:expr)),*) => {
        $(
            allow_c!(($x, $table_name, $view_name, $register_columns, $place_holder));
            allow_r!(($x, $view_name));
            allow_u!(($x, $table_name, $update_set_clause));
        )*
    };
}
macro_rules! allow_crd {
    ($(($x: ty, $table_name:expr, $view_name:expr, $register_columns:expr, $place_holder:expr)),*) => {
        $(
            allow_c!(($x, $table_name, $view_name, $register_columns, $place_holder));
            allow_r!(($x, $view_name));
            allow_d!(($x, $table_name));
        )*
    };
}

allow_cru!(
    (
        Paragraph,
        "paragraph",
        "view_deserializable_paragraph",
        "item_id,headline_id,paragraph_pos,accepted_draft_id",
        4,
        "headline_id=?,paragraph_pos=?,accepted_draft_id=?"
    ),
    (
        Headline,
        "headlines",
        "headlines",
        "item_id,parent_id,headline_pos",
        3,
        "parent_id=?,headline_pos=?"
    )
);

allow_crud!(
    (
        BackgroundInfo,
        "background_info",
        "background_info",
        "body",
        1,
        "body=?"
    ),
    (Tag, "tags", "tags", "name,memo", 2, "name=?,memo=?"),
    (
        Publisher,
        "publishers",
        "publishers",
        "name,memo",
        2,
        "name=?,memo=?"
    ),
    (
        Bibliography,
        "bibliographies",
        "view_deserializable_bibliographies",
        "isbn,url,title,detail,publisher_id,publication_date,tmp_registration_id",
        7,
        "isbn=?,url=?,title=?,detail=?,publisher_id=?,publication_date=?"
    ),
    (
        BibliographyAuthor,
        "bibliography_authors",
        "bibliography_authors",
        "name,memo",
        2,
        "name=?,memo=?"
    ),
    (
        Item,
        "items",
        "view_deserializable_item",
        "item_type,title",
        2,
        "title=?"
    ),
    (
        Draft,
        "draft",
        "draft",
        "paragraph_id,draft_pos,title,body",
        4,
        "paragraph_id=?,draft_pos=?,title=?,body=?"
    ),
    (
        ParagraphSummary,
        "paragraph_summaries",
        "paragraph_summaries",
        "paragraph_id,title,detail",
        3,
        "paragraph_id=?,title=?,detail=?"
    ),
    (
        BackgroundReference,
        "background_references",
        "view_deserializable_background_reference",
        "background_info_id,bibliography_id,location",
        3,
        "background_info_id=?,bibliography_id=?,location=?"
    ),
    (
        ItemReference,
        "item_references",
        "view_deserializable_item_reference",
        "item_id,bibliography_id,location",
        3,
        "item_id=?,bibliography_id=?,location=?"
    ),
    (
        TaskCategory,
        "task_categories",
        "task_categories",
        "name,autocomplete_paragraph_link",
        2,
        "name=?,autocomplete_paragraph_link=?"
    ),
    (
        TaskTemplate,
        "task_templates",
        "view_deserializable_task_template",
        "task_category_id,title,detail",
        3,
        "task_category_id=?,title=?,detail=?"
    ),
    (
        Task,
        "tasks",
        "view_deserializable_task",
        "item_id,task_category_id,title,detail,is_finished",
        5,
        "item_id=?,task_category_id=?,title=?,detail=?,is_finished=?"
    ),
    (
        ParagraphLink,
        "paragraph_link",
        "view_deserializable_paragraph_link",
        "from_paragraph_id,to_paragraph_id,task_id,comment",
        4,
        "from_paragraph_id=?,to_paragraph_id=?,task_id=?,comment=?"
    ),
    (
        Setting,
        "settings",
        "settings",
        "setting_key,setting_value",
        2,
        "setting_value=?"
    ),
    (
        RelBibliographyAuthor,
        "rel_bibliography_authors",
        "rel_bibliography_authors",
        "bibliography_id,bibliography_author_id",
        2,
        "bibliography_id=?,bibliography_author_id=?"
    ),
    (
        RelTagAndItem,
        "rel_tag_and_item",
        "rel_tag_and_item",
        "item_id,tag_id",
        2,
        "item_id=?,tag_id=?"
    ),
    (
        RelBackgroundAndItem,
        "rel_background_and_item",
        "rel_background_and_item",
        "item_id,background_info_id",
        2,
        "item_id=?,background_info_id=?"
    )
);

allow_crud!((
    BookSearchApi,
    "book_search_api",
    "book_search_api",
    "name,detail,isbn_url,text_url,mapping_script",
    5,
    "name=?,detail=?,isbn_url=?,text_url=?,mapping_script=?"
));
