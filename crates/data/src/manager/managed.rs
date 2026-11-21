use crate::manager::DataManager;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// `PrehniteManaged<T>` で扱えることを表すマーカートレイト
pub trait PrehniteManagedItem: for<'de> Deserialize<'de> + Serialize {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "T: PrehniteManagedItem"))]
pub struct PrehniteManaged<T> {
    manager: DataManager,
    item: T,
}

impl<T: PrehniteManagedItem> Deref for PrehniteManaged<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.as_ref()
    }
}

impl<T> AsRef<T> for PrehniteManaged<T> {
    fn as_ref(&self) -> &T {
        &self.item
    }
}

impl<T: PrehniteManagedItem> PrehniteManaged<T> {
    pub(crate) fn new(manager: DataManager, item: T) -> Self {
        Self { manager, item }
    }

    pub(crate) fn manager(&self) -> DataManager {
        self.manager
    }

    pub(crate) fn as_mut(&mut self) -> &mut T {
        &mut self.item
    }
}
