use crate::database::zstd_compressed_blob::ZstdCompressedBlob;
use crate::{Document, DocumentId};
use sqlx::{FromRow, SqliteConnection, query_as};

#[allow(unused)]
#[derive(FromRow, Debug, Clone)]
pub(crate) struct DbDocumentHistory {
    pub id: i64,
    pub document_id: DocumentId,
    pub title: ZstdCompressedBlob,
    pub content: ZstdCompressedBlob,
    pub created_at: jiff_sqlx::Timestamp,
    pub updated_at: jiff_sqlx::Timestamp,
}

#[allow(unused)]
impl DbDocumentHistory {
    pub(crate) async fn create(
        document: Document,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        Ok(query_as("INSERT INTO document_histories(document_id, title, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?) RETURNING *")
            .bind(document.document_id)
            .bind(document.title.as_ref().map(|x|x.as_str()))
            .bind(document.content.as_str())
            .bind(jiff_sqlx::Timestamp::from(document.created_at))
            .bind(jiff_sqlx::Timestamp::from(document.updated_at))
            .fetch_one(conn).await?)
    }
}
