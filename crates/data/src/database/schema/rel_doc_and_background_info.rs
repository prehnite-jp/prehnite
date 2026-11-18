use crate::database::schema::DocumentId;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqliteConnection, query, query_as};

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct DbRelDocsAndBackgroundInfo {
    pub document_id: DocumentId,
    pub bg_info_document_id: DocumentId,
}

impl DbRelDocsAndBackgroundInfo {
    pub(crate) async fn create(
        document_id: DocumentId,
        bg_info_document_id: DocumentId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Option<Self>> {
        Ok(
            query_as(
                "INSERT INTO rel_docs_and_background_info(document_id, bg_info_document_id) VALUES (?, ?) ON CONFLICT DO NOTHING RETURNING *",
            )
            .bind(document_id)
            .bind(bg_info_document_id)
            .fetch_optional(conn)
            .await?,
        )
    }

    pub(crate) async fn delete(
        document_id: DocumentId,
        bg_info_document_id: DocumentId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<bool> {
        Ok(
            query("DELETE FROM rel_docs_and_background_info WHERE document_id = ? AND bg_info_document_id = ?")
                .bind(document_id)
                .bind(bg_info_document_id)
                .execute(conn)
                .await?
                .rows_affected()
                > 0,
        )
    }

    pub(crate) async fn fetch_background_info(
        document_id: DocumentId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Vec<DocumentId>> {
        Ok(query_as::<_, DbRelDocsAndBackgroundInfo>(
            "SELECT * FROM rel_docs_and_background_info WHERE document_id = ? ",
        )
        .bind(document_id)
        .fetch_all(conn)
        .await?
        .into_iter()
        .map(|x| x.bg_info_document_id)
        .collect())
    }
    pub(crate) async fn fetch_docs(
        bg_info_document_id: DocumentId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Vec<DocumentId>> {
        Ok(
            query_as::<_, DbRelDocsAndBackgroundInfo>("SELECT * FROM rel_docs_and_background_info WHERE bg_info_document_id = ? ")
                .bind(bg_info_document_id)
                .fetch_all(conn)
                .await?
                .into_iter()
                .map(|x| x.document_id)
                .collect(),
        )
    }
}
