use crate::inner_data_manager::InnerDataManager;
use crate::{PrehniteManaged, PrehniteManagedItem};
use sqlx::pool::PoolConnection;
use sqlx::{Sqlite, SqliteConnection};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Weak};
use tracing::warn;

pub(crate) struct ManagedConnection(Arc<InnerDataManager>, PoolConnection<Sqlite>);

impl Deref for ManagedConnection {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl DerefMut for ManagedConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.1
    }
}

impl From<(Arc<InnerDataManager>, PoolConnection<Sqlite>)> for ManagedConnection {
    fn from(value: (Arc<InnerDataManager>, PoolConnection<Sqlite>)) -> Self {
        Self(value.0, value.1)
    }
}

impl ManagedConnection {
    pub(crate) fn manager(&self) -> &InnerDataManager {
        &self.0
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct WeakInnerDataManager(Weak<InnerDataManager>);

impl WeakInnerDataManager {
    pub(crate) fn get(&self) -> crate::Result<Arc<InnerDataManager>> {
        if let Some(x) = Weak::upgrade(&self.0) {
            Ok(x)
        } else {
            // データマネージャは既に閉じられています。
            warn!("The data manager has already been closed.");
            Err(crate::Error::DataManagerClosed)
        }
    }

    pub(crate) fn manage<T: PrehniteManagedItem>(&self, item: T) -> PrehniteManaged<T> {
        PrehniteManaged::new(self.clone(), item)
    }

    pub(crate) async fn acquire(&self) -> crate::Result<ManagedConnection> {
        let manager = self.get()?;
        let conn = manager.acquire().await?;
        Ok((manager, conn).into())
    }
}

impl TryFrom<&WeakInnerDataManager> for Arc<InnerDataManager> {
    type Error = crate::Error;

    fn try_from(value: &WeakInnerDataManager) -> Result<Self, Self::Error> {
        value.get()
    }
}

impl From<Weak<InnerDataManager>> for WeakInnerDataManager {
    fn from(value: Weak<InnerDataManager>) -> Self {
        Self(value)
    }
}
