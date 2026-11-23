use crate::data_options::PrehniteDataOptions;
use crate::database::schema::deleted_documents::DbDeletedDocument;
use crate::database::schema::document::DbDocument;
use crate::manager::inner_data_manager::InnerDataManager;
use crate::{BinderRepository, DocumentRepository, TagRepository};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::pool::PoolConnection;
use sqlx::{Acquire, Sqlite};
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use tantivy::aggregation::agg_result::{AggregationResult, MetricResult};
use tantivy::aggregation::{AggContextParams, AggregationCollector};

pub mod document_index;
pub mod inner_data_manager;
pub mod managed;
pub mod weak_inner_data_manager;

use crate::weak_inner_data_manager::WeakInnerDataManager;
pub use document_index::*;
pub use managed::*;

#[derive(Deserialize, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SyncStep {
    Delete,
    Upsert,
}

#[derive(Clone, Debug)]
pub struct DataManager(Arc<InnerDataManager>);

impl Deref for DataManager {
    type Target = InnerDataManager;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DataManager {
    pub(crate) fn get_manager(&self) -> Arc<InnerDataManager> {
        self.0.clone()
    }

    pub(crate) async fn acquire(&self) -> crate::Result<PoolConnection<Sqlite>> {
        self.get_manager().acquire().await
    }

    pub async fn open_or_create(
        db_path: impl AsRef<Path>,
        data_options: Option<PrehniteDataOptions>,
    ) -> crate::Result<Self> {
        let inner = Arc::new(InnerDataManager::open_or_create(db_path, data_options).await?);
        Ok(Self(inner))
    }

    fn weak_manager(&self) -> WeakInnerDataManager {
        Arc::downgrade(&self.0).into()
    }

    #[tracing::instrument]
    pub fn binders(&self) -> BinderRepository {
        BinderRepository::new(self.weak_manager())
    }

    #[tracing::instrument]
    pub fn documents(&self) -> DocumentRepository {
        DocumentRepository::new(self.weak_manager())
    }

    #[tracing::instrument]
    pub fn tags(&self) -> TagRepository {
        TagRepository::new(self.weak_manager())
    }

    pub(crate) fn manage<T: PrehniteManagedItem>(&self, item: T) -> PrehniteManaged<T> {
        PrehniteManaged::new(self.weak_manager(), item)
    }

    // TODO: ファイルが複製されたとき、インデックスが不整合となる場合がある。これはインデックスを再生成することで解決できる。
    #[tracing::instrument]
    pub async fn sync_document_index(&self) -> crate::Result<()> {
        let mgr = self.get_manager();
        let reader = mgr.document_index.reader()?;
        let searcher = reader.searcher();
        let docs = searcher.search(
            &mgr.document_index.parse_query("")?,
            &AggregationCollector::from_aggs(
                serde_json::from_value(
                    json!({"aggs":{"max_updated_at": { "max": { "field": "updated_at" }}}}),
                )?,
                AggContextParams::default(),
            ),
        )?;
        let last_updated_at = Timestamp::from_second(
            if let Some(AggregationResult::MetricResult(MetricResult::Max(updated_at))) =
                docs.0.get("max_updated_at")
            {
                updated_at.value.unwrap_or_default() as i64
            } else {
                0
            },
        )?;
        let mut conn = self.acquire().await?;
        let mut tx = conn.begin().await?;
        let idx = &self.get_manager().document_index;
        let deleted_documents =
            DbDeletedDocument::fetch_diff_all(last_updated_at, &mut *tx).await?;
        for i in deleted_documents {
            idx.delete(i.document_id)?;
        }
        let updated_documents = DbDocument::fetch_diff_all(last_updated_at, &mut *tx).await?;
        for i in updated_documents {
            idx.upsert(i.into())?;
        }
        Ok(())
    }
}
