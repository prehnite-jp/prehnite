use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{FromRow, QueryBuilder, SqliteConnection, query, query_as};
use std::collections::BTreeMap;
use strum::VariantArray;

#[derive(
    Deserialize, Serialize, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, VariantArray,
)]
pub enum DbMetadataKey {
    PrehniteId,
    DataFormatVersion,
}

impl DbMetadataKey {
    fn issue_prehnite_id() -> Option<String> {
        Some(uuid::Uuid::new_v4().to_string())
    }

    fn default_metadata_key() -> Option<String> {
        Some(env!("CARGO_PKG_VERSION").into())
    }

    pub(crate) fn default_value(&self) -> Option<String> {
        match self {
            DbMetadataKey::PrehniteId => Self::issue_prehnite_id(),
            DbMetadataKey::DataFormatVersion => Self::default_metadata_key(),
        }
    }
}

#[derive(FromRow, Debug, Clone)]
pub(crate) struct DbMetadata {
    pub name: Json<DbMetadataKey>,
    pub value: Option<String>,
}

impl DbMetadata {
    pub(crate) async fn init_metadata(conn: &mut SqliteConnection) -> sqlx::Result<bool> {
        if Self::exists(DbMetadataKey::DataFormatVersion, conn).await? {
            Ok(false)
        } else {
            Ok(
                QueryBuilder::new("INSERT INTO metadata(name, value) VALUES ")
                    .push_values(DbMetadataKey::VARIANTS, |mut sep, itm| {
                        sep.push_bind(Json(itm.clone()))
                            .push_bind(itm.default_value());
                    })
                    .push(" ON CONFLICT DO NOTHING;")
                    .build()
                    .execute(conn)
                    .await?
                    .rows_affected()
                    > 0,
            )
        }
    }

    pub(crate) async fn fetch(
        name: DbMetadataKey,
        conn: &mut SqliteConnection,
    ) -> sqlx::Result<Self> {
        query_as("SELECT * FROM metadata WHERE name = ?")
            .bind(Json(name))
            .fetch_one(conn)
            .await
    }

    pub(crate) async fn fetch_all(
        conn: &mut SqliteConnection,
    ) -> sqlx::Result<BTreeMap<DbMetadataKey, Option<String>>> {
        Ok(query_as::<_, Self>("SELECT * FROM metadata")
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|x| (x.name.0, x.value))
            .collect())
    }

    pub(crate) async fn update(
        name: DbMetadataKey,
        value: Option<String>,
        conn: &mut SqliteConnection,
    ) -> sqlx::Result<Self> {
        let name = Json(name);
        Self { name, value }.save(conn).await
    }

    pub(crate) async fn save(&self, conn: &mut SqliteConnection) -> sqlx::Result<Self> {
        query_as("INSERT INTO metadata(name, value) VALUES (?1, ?2) ON CONFLICT DO UPDATE SET value = ?2 WHERE name = ?1 RETURNING *")
            .bind(self.name)
            .bind(&self.value)
            .fetch_one(conn)
            .await
    }

    pub(crate) async fn exists(
        name: DbMetadataKey,
        conn: &mut SqliteConnection,
    ) -> sqlx::Result<bool> {
        Ok(query("SELECT * FROM metadata WHERE name = ?")
            .bind(Json(name))
            .fetch_optional(conn)
            .await?
            .is_some())
    }
}
