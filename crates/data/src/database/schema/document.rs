use crate::DocumentId;
use crate::database::zstd_compressed_blob::ZstdCompressedBlob;
use jiff_sqlx::ToSqlx;
use prehnite_time_bridge::jiff::Timestamp;
use sqlx::{FromRow, SqliteConnection, query, query_as};

#[derive(FromRow, Debug, Clone)]
pub(crate) struct DbDocument {
    pub id: DocumentId,
    pub title: Option<ZstdCompressedBlob>,
    pub content: ZstdCompressedBlob,
    pub deleted: bool,
    pub created_at: jiff_sqlx::Timestamp,
    pub updated_at: jiff_sqlx::Timestamp,
}

impl DbDocument {
    pub(crate) async fn fetch(
        document_id: DocumentId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        Ok(query_as("SELECT * FROM documents WHERE id = ?")
            .bind(document_id)
            .fetch_one(conn)
            .await?)
    }

    pub(crate) async fn create(
        conn: &mut SqliteConnection,
        title: String,
        content: String,
    ) -> crate::Result<Self> {
        let now = jiff_sqlx::Timestamp::from(Timestamp::now());
        Ok(query_as("INSERT INTO documents(title, content, deleted, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4) RETURNING *")
            .bind(ZstdCompressedBlob::from(title.as_bytes().to_vec()))
            .bind(ZstdCompressedBlob::from(content.as_bytes().to_vec()))
            .bind(false)
            .bind(now)
            .fetch_one(conn)
            .await?)
    }

    pub(crate) async fn delete(
        document_id: DocumentId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<bool> {
        Ok(query("DELETE FROM documents WHERE id = ?")
            .bind(document_id)
            .execute(conn)
            .await?
            .rows_affected()
            > 0)
    }

    pub(crate) async fn save(&self, conn: &mut SqliteConnection) -> crate::Result<bool> {
        Ok(query(
            "UPDATE documents SET title = ?, content = ?, deleted = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&self.title)
        .bind(&self.content)
        .bind(self.deleted)
        .bind(self.updated_at)
        .bind(self.id)
        .execute(&mut *conn)
        .await?
        .rows_affected()
            > 0)
    }

    pub(crate) async fn fetch_diff_all(
        last_updated_at: Timestamp,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Vec<Self>> {
        Ok(
            query_as("SELECT * FROM documents WHERE updated_at > ? AND NOT deleted")
                .bind(last_updated_at.to_sqlx())
                .fetch_all(conn)
                .await?,
        )
    }
}
