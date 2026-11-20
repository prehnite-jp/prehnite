use crate::manager::DataManager;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// `PrehniteManaged<T>` で扱えることを表すマーカートレイト
pub trait PrehniteManagedItem: for<'de> Deserialize<'de> + Serialize {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound(deserialize = "T: PrehniteManagedItem"))]
pub struct PrehniteManaged<T>
{
    manager: DataManager,
    item: T,
}

impl<T: PrehniteManagedItem> Deref for PrehniteManaged<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: PrehniteManagedItem> PrehniteManaged<T> {
    pub(crate) fn new(manager: DataManager, item: T) -> Self {
        Self { manager, item }
    }

    pub(crate) fn item(self) -> T {
        self.item
    }

    pub(crate) fn manager(&self) -> DataManager {
        self.manager
    }

    pub(crate) fn as_mut(&mut self) -> &mut T {
        &mut self.item
    }
}
