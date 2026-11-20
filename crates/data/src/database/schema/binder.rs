use crate::BinderId;
use jiff_sqlx::ToSqlx;
use prehnite_time_bridge::jiff;
use prehnite_time_bridge::jiff::Timestamp;
use sqlx::{FromRow, QueryBuilder, SqliteConnection, query, query_as};

#[derive(FromRow, Debug, Clone)]
pub(crate) struct DbBinder {
    pub id: BinderId,
    pub title: String,
    pub details: Option<String>,
    pub created_at: jiff_sqlx::Timestamp,
    pub updated_at: jiff_sqlx::Timestamp,
}

impl DbBinder {
    pub(crate) async fn create(
        title: String,
        details: Option<String>,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        Ok(query_as(
            "INSERT INTO binders(title, details, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
        )
        .bind(title)
        .bind(details)
        .bind(Timestamp::now().to_sqlx())
        .fetch_one(conn)
        .await?)
    }

    pub(crate) async fn delete(&self, conn: &mut SqliteConnection) -> crate::Result<bool> {
        Ok(query("DELETE FROM binders WHERE id = ?")
            .bind(self.id)
            .execute(conn)
            .await?
            .rows_affected()
            > 0)
    }

    pub(crate) async fn save(&self, conn: &mut SqliteConnection) -> crate::Result<()> {
        query("UPDATE binders SET title = ?, details = ?, updated_at = ? WHERE id = ?")
            .bind(&self.title)
            .bind(&self.details)
            .bind(jiff::Timestamp::now().to_sqlx())
            .bind(self.id)
            .execute(conn)
            .await?;
        Ok(())
    }

    pub(crate) async fn fetch(
        id: BinderId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Option<Self>> {
        Ok(query_as("SELECT * FROM binders WHERE id = ?")
            .bind(id)
            .fetch_optional(conn)
            .await?)
    }

    pub(crate) async fn search(
        title: Option<String>,
        details: Option<String>,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Vec<Self>> {
        let mut builder = QueryBuilder::new("SELECT * FROM binders WHERE ");
        let mut all = true;
        let mut sep = builder.separated(" AND ");
        if let Some(title) = title {
            sep.push(" title LIKE ?").push_bind(title);
            all = false;
        }
        if let Some(details) = details {
            sep.push(" details LIKE ?").push_bind(details);
            all = false;
        }
        if all {
            sep.push("TRUE");
        }
        sep.push_unseparated("");
        Ok(builder.build_query_as().fetch_all(conn).await?)
    }
}
