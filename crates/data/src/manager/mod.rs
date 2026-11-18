use crate::data_options::PrehniteDataOptions;
use crate::deleted_documents::DbDeletedDocument;
use crate::document::DbDocument;
use crate::manager::inner_data_manager::InnerDataManager;
use crate::manager::managed::{PrehniteManaged, PrehniteManagedItem};
use crate::manager::registry::ManagerRegistry;
use crate::repos::binder::BinderRepository;
use crate::repos::document::DocumentRepository;
use crate::repos::tag::TagRepository;
use prehnite_time_bridge::jiff::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::pool::PoolConnection;
use sqlx::{Acquire, Sqlite};
use std::ops::AddAssign;
use std::path::Path;
use std::sync::{Arc, LazyLock, Mutex};
use tantivy::aggregation::agg_result::{AggregationResult, MetricResult};
use tantivy::aggregation::{AggContextParams, AggregationCollector};
use tracing::warn;

pub mod document_index;
pub mod inner_data_manager;
pub mod managed;
pub mod registry;

#[derive(Deserialize, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SyncStep {
    Delete,
    Upsert,
}

#[derive(Deserialize, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct DataManager(u64);

impl DataManager {
    pub(in crate::manager) fn issue() -> DataManager {
        static NEXT: LazyLock<Mutex<u64>> = LazyLock::new(Default::default);
        let mut locked = NEXT.lock().unwrap();
        let id = *locked;
        locked.add_assign(1);
        DataManager(id)
    }

    pub(crate) fn get_manager(&self) -> crate::Result<Arc<InnerDataManager>> {
        if let Some(mgr) = ManagerRegistry::get(self) {
            Ok(mgr)
        } else {
            // データマネージャは既に閉じられています。
            warn!("The data manager has already been closed.");
            Err(crate::Error::DataManagerClosed)
        }
    }

    pub(crate) async fn acquire(&self) -> crate::Result<PoolConnection<Sqlite>> {
        self.get_manager()?.acquire().await
    }

    pub fn close(&self) -> bool {
        ManagerRegistry::close(self)
    }

    pub fn is_closed(&self) -> bool {
        ManagerRegistry::is_closed(self)
    }

    pub async fn open_or_create(
        db_path: impl AsRef<Path>,
        data_options: Option<PrehniteDataOptions>,
    ) -> crate::Result<Self> {
        let inner = Arc::new(InnerDataManager::open_or_create(db_path, data_options).await?);
        Ok(ManagerRegistry::open(inner))
    }

    #[tracing::instrument]
    pub fn binders(&self) -> BinderRepository {
        BinderRepository::new(*self)
    }

    #[tracing::instrument]
    pub fn documents(&self) -> DocumentRepository {
        DocumentRepository::new(*self)
    }

    #[tracing::instrument]
    pub fn tags(&self) -> TagRepository {
        TagRepository::new(*self)
    }

    pub(crate) fn manage<T: PrehniteManagedItem>(&self, item: T) -> PrehniteManaged<T> {
        PrehniteManaged::new(*self, item)
    }

    // TODO: ファイルが複製された場合、インデックスが不整合となる。それを回避する方法を考える
    // 案1: グローバルにSqliteDbを作成。最終更新レコードの日時を持ったテーブルをグローバルに持っておき、Idとそれが一致する場合、同一ファイルとみなす。異なればIdを再生成し、新たにインデックスを作成する。
    #[tracing::instrument]
    pub async fn sync_document_index(&self) -> crate::Result<()> {
        let mgr = self.get_manager()?;
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
        let idx = &self.get_manager()?.document_index;
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
