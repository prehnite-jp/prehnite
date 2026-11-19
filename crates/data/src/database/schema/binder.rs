use crate::database::schema::BinderId;
use jiff_sqlx::ToSqlx;
use prehnite_time_bridge::jiff;
use sqlx::{FromRow, SqliteConnection, query};

#[derive(FromRow, Debug, Clone)]
pub(crate) struct DbBinder {
    pub id: BinderId,
    pub title: String,
    pub details: Option<String>,
    pub created_at: jiff_sqlx::Timestamp,
    pub updated_at: jiff_sqlx::Timestamp,
}

impl DbBinder {
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
}
