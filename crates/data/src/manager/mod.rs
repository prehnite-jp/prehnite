use crate::data_options::PrehniteDataOptions;
use crate::database::schema::document::DbDocument;
use crate::manager::inner_data_manager::InnerDataManager;
use crate::{BinderRepository, DocumentRepository, TagRepository};
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{Acquire, Sqlite};
use std::cmp::Ordering;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

pub mod document_index;
pub mod inner_data_manager;
pub mod managed;
pub mod weak_inner_data_manager;

use crate::manager::weak_inner_data_manager::WeakInnerDataManager;
pub use document_index::*;
#[cfg(feature = "gpui")]
pub use _gpui_impl::DataManagerStore;
pub use managed::*;

#[derive(Deserialize, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SyncStep {
    Delete,
    Upsert,
}

#[derive(Debug, PartialEq, Eq)]
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

    fn drop_document_index(&self) -> crate::Result<()> {
        let mut w = self.document_index.writer.lock().unwrap();
        w.delete_all_documents()?;
        w.commit()?;
        Ok(())
    }

    /// インデックスを再生成します。
    #[tracing::instrument]
    pub async fn regenerate_document_index(&self) -> crate::Result<()> {
        let mut conn = self.acquire().await?;
        let mut tx = conn.begin().await?;
        let idx = &self.document_index;
        self.drop_document_index()?;
        let updated_documents = DbDocument::fetch_all(&mut *tx).await?;
        for i in updated_documents {
            idx.upsert(i.into())?;
        }
        Ok(())
    }
}
#[cfg(feature = "gpui")]
mod _gpui_impl {
    use crate::TagStore;
    use crate::manager::DataManager;
    use gpui_kit::Global;
    use std::ops::Deref;
    use std::sync::Arc;
    use tracing_unwrap::ResultExt;

    #[derive(Debug, Default)]
    pub struct DataManagerStore(Option<Arc<DataManager>>);
    impl DataManagerStore {
        pub(crate) fn global(cx: &mut gpui_kit::App) -> &DataManagerStore {
            cx.global()
        }

        pub(crate) fn init(cx: &mut gpui_kit::App) {
            cx.set_global(Self::default());
            cx.observe_global::<Self>(move |cx| {
                TagStore::reload(cx).ok_or_log();
            })
            .detach();
        }

        pub fn update(cx: &mut gpui_kit::App, manager: DataManager) -> crate::Result<()> {
            cx.set_global(Self(Some(Arc::new(manager))));
            Ok(())
        }

        pub fn close(cx: &mut gpui_kit::App) {
            cx.set_global(Self::default());
            TagStore::clear(cx);
        }

        pub fn get(cx: &mut gpui_kit::App) -> crate::Result<&DataManager> {
            Self::global(cx)
                .0
                .as_ref()
                .map(Deref::deref)
                .ok_or(crate::Error::GlobalDataManagerNotOpened)
        }

        pub(crate) fn get_owned(cx: &mut gpui_kit::App) -> crate::Result<Arc<DataManager>> {
            Self::global(cx)
                .0
                .clone()
                .ok_or(crate::Error::GlobalDataManagerNotOpened)
        }
    }

    impl Global for DataManagerStore {}
}
