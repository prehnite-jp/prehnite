use crate::database::zstd_compressed_blob::ZstdCompressedBlob;
use crate::database::schema::DocumentId;
use crate::models::document::Document;
use sqlx::{FromRow, SqliteConnection, query_as};

#[derive(FromRow, Debug, Clone)]
pub(crate) struct DbDocumentHistory {
    pub id: i64,
    pub document_id: DocumentId,
    pub title: ZstdCompressedBlob,
    pub content: ZstdCompressedBlob,
    pub created_at: jiff_sqlx::Timestamp,
    pub updated_at: jiff_sqlx::Timestamp,
}

impl DbDocumentHistory {
    pub(crate) async fn create(
        document: Document,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        Ok(query_as("INSERT INTO document_histories(document_id, title, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?) RETURNING *")
            .bind(document.document_id)
            .bind(document.title)
            .bind(document.content)
            .bind(jiff_sqlx::Timestamp::from(document.created_at))
            .bind(jiff_sqlx::Timestamp::from(document.updated_at))
            .fetch_one(conn).await?)
    }
}
