use crate::{TagId, TagMetadata};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{FromRow, SqliteConnection, query, query_as};

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub struct DbTag {
    pub id: TagId,
    pub title: String,
    pub details: Option<String>,
    pub metadata: Json<TagMetadata>,
}

impl DbTag {
    pub(crate) async fn fetch_or_create(
        title: impl Into<String>,
        details: Option<impl Into<String>>,
        metadata: Option<impl Into<TagMetadata>>,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        let title = title.into();
        let details = details.map(Into::into);
        let metadata = metadata.map(Into::into).unwrap_or_default();
        Ok(if let Some(v) = Self::fetch(&title, conn).await? {
            v
        } else {
            query_as("INSERT INTO tags(title, details, metadata) VALUES (?, ?, ?) ON CONFLICT DO NOTHING RETURNING *")
                .bind(title)
                .bind(details)
                .bind(Json(metadata))
                .fetch_one(conn)
                .await?
        })
    }

    pub(crate) async fn fetch(
        title: impl Into<String>,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Option<Self>> {
        Ok(query_as("SELECT * FROM tags WHERE title = ?")
            .bind(title.into())
            .fetch_optional(conn)
            .await?)
    }

    pub(crate) async fn fetch_by_id(
        id: TagId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Option<Self>> {
        Ok(query_as("SELECT * FROM tags WHERE id = ?")
            .bind(id)
            .fetch_optional(conn)
            .await?)
    }

    pub(crate) async fn fetch_all(conn: &mut SqliteConnection) -> crate::Result<Vec<Self>> {
        Ok(query_as("SELECT * FROM tags").fetch_all(conn).await?)
    }

    pub(crate) async fn save(&self, conn: &mut SqliteConnection) -> crate::Result<()> {
        query("UPDATE tags SET title = ?, details = ?, metadata = ? WHERE id = ?")
            .bind(&self.title)
            .bind(&self.details)
            .bind(&self.metadata)
            .bind(self.id)
            .execute(conn)
            .await?;
        Ok(())
    }

    pub(crate) async fn delete(&self, conn: &mut SqliteConnection) -> crate::Result<bool> {
        Ok(query("DELETE FROM tags WHERE id = ?")
            .bind(self.id)
            .execute(conn)
            .await?
            .rows_affected()
            > 0)
    }
}
