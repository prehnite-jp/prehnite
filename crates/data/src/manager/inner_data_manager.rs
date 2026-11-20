use crate::data_options::PrehniteDataOptions;
use crate::manager::document_index::DocumentIndex;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Sqlite, SqlitePool};
use std::path::{Path, PathBuf};
use crate::database::schema::metadata::DbMetadata;

async fn connect_database(path: impl AsRef<Path>) -> crate::Result<SqlitePool> {
    // DBへの接続を確立
    let pool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename(path)
                .create_if_missing(true),
        )
        .await?;
    let mut conn = pool.acquire().await?;
    // マイグレーション
    const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();
    MIGRATOR.run(&mut *conn).await?;

    // メタデータを設定
    DbMetadata::init_metadata(&mut *conn).await?;
    Ok(pool)
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct InnerDataManager {
    pub(crate) database_path: PathBuf,
    pub(crate) database_pool: SqlitePool,
    pub(crate) data_options: PrehniteDataOptions,
    pub(crate) document_index: DocumentIndex,
}

impl InnerDataManager {
    pub(crate) async fn open_or_create(
        db_path: impl AsRef<Path>,
        data_options: Option<PrehniteDataOptions>,
    ) -> crate::Result<Self> {
        let database_path = db_path.as_ref().to_path_buf();
        let database_pool = connect_database(&db_path).await?;
        let data_options = data_options.unwrap_or_default();
        let document_index =
            DocumentIndex::open_or_create(&data_options, &mut *database_pool.acquire().await?).await?;

        Ok(InnerDataManager {
            database_path,
            database_pool,
            data_options,
            document_index,
        })
    }

    pub(crate) async fn acquire(&self) -> crate::Result<PoolConnection<Sqlite>> {
        Ok(self.database_pool.acquire().await?)
    }
}
