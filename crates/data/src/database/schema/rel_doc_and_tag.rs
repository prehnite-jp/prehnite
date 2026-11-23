use crate::{DocumentId, TagId};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqliteConnection, query, query_as};

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct DbRelDocsAndTags {
    pub document_id: DocumentId,
    pub tag_id: TagId,
}

impl DbRelDocsAndTags {
    pub(crate) async fn create(
        document_id: DocumentId,
        tag_id: TagId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Option<Self>> {
        Ok(
            query_as(
                "INSERT INTO rel_docs_and_tags(document_id, tag_id) VALUES (?, ?) ON CONFLICT DO NOTHING RETURNING *",
            )
            .bind(document_id)
            .bind(tag_id)
            .fetch_optional(conn)
            .await?,
        )
    }

    pub(crate) async fn delete(
        document_id: DocumentId,
        tag_id: TagId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<bool> {
        Ok(
            query("DELETE FROM rel_docs_and_tags WHERE document_id = ? AND tag_id = ?")
                .bind(document_id)
                .bind(tag_id)
                .execute(conn)
                .await?
                .rows_affected()
                > 0,
        )
    }

    pub(crate) async fn fetch_tags(
        document_id: DocumentId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Vec<TagId>> {
        Ok(query_as::<_, DbRelDocsAndTags>(
            "SELECT * FROM rel_docs_and_tags WHERE document_id = ? ",
        )
        .bind(document_id)
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|x| x.tag_id)
        .collect())
    }
    pub(crate) async fn fetch_docs(
        tags: Vec<&str>,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Vec<DocumentId>> {
        let mut sql_query = sqlx::QueryBuilder::new(
            "SELECT * FROM rel_docs_and_tags WHERE tag_id IN ( SELECT id FROM tags WHERE title IN ( ",
        );
        let mut sep = sql_query.separated(", ");
        for tag in tags.iter() {
            sep.push_bind(tag);
        }
        sep.push_unseparated(") ");
        sql_query
            .push(") GROUP BY document_id HAVING COUNT(DISTINCT tag_id) = ")
            .push_bind(tags.len() as i64);
        Ok(sql_query
            .build_query_as::<DbRelDocsAndTags>()
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|x| x.document_id)
            .collect())
    }
}
