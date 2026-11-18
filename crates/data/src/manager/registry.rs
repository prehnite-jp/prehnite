use crate::manager::{DataManager, InnerDataManager};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

static REGISTRY: LazyLock<RwLock<HashMap<DataManager, Arc<InnerDataManager>>>> =
    LazyLock::new(Default::default);

#[derive(Default)]
pub(crate) struct ManagerRegistry;

impl ManagerRegistry {
    pub(crate) fn open(manager: Arc<InnerDataManager>) -> DataManager {
        let id = DataManager::issue();
        REGISTRY.write().unwrap().insert(id, manager);
        id
    }

    pub(crate) fn get(id: &DataManager) -> Option<Arc<InnerDataManager>> {
        REGISTRY.read().unwrap().get(id).cloned()
    }

    pub(crate) fn is_closed(id: &DataManager) -> bool {
        REGISTRY.read().unwrap().contains_key(id)
    }

    pub(crate) fn close(id: &DataManager) -> bool {
        REGISTRY.write().unwrap().remove(id).is_some()
    }
}
