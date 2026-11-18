use crate::DocumentId;
use jiff_sqlx::ToSqlx;
use prehnite_time_bridge::jiff;
use sqlx::{FromRow, SqliteConnection, query_as};

#[derive(FromRow, Debug, Clone)]
pub(crate) struct DbDeletedDocument {
    pub document_id: DocumentId,
    pub deleted_at: jiff_sqlx::Timestamp,
}

impl DbDeletedDocument {
    /// ドキュメントの最終更新日時以降に削除されたドキュメント(論理削除済み含む)を取得する。
    pub(crate) async fn fetch_diff_all(
        last_updated_at: jiff::Timestamp,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Vec<Self>> {
        Ok(query_as(
            "
SELECT document_id, deleted_at 
FROM deleted_documents 
WHERE deleted_at >= ?1 
UNION ALL 
SELECT id AS document_id, updated_at AS deleted_at
FROM documents 
WHERE updated_at >= ?1 AND deleted;
",
        )
        .bind(last_updated_at.to_sqlx())
        .fetch_all(conn)
        .await?)
    }
}
