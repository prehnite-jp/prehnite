use crate::BinderId;
use crate::manager::DataManager;
use crate::manager::managed::PrehniteManaged;
use crate::models::binder::Binder;

pub struct BinderRepository {
    manager: DataManager,
}

impl BinderRepository {
    pub fn new(manager: DataManager) -> Self {
        Self { manager }
    }

    pub async fn create(
        &self,
        title: String,
        description: Option<String>,
    ) -> crate::Result<PrehniteManaged<Binder>> {
        todo!()
    }

    pub async fn search_by_title(
        &self,
        title: String,
    ) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        todo!()
    }
    pub async fn search_by_description(
        &self,
        description: String,
    ) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        todo!()
    }
    pub async fn search(&self, keyword: String) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        todo!()
    }

    pub async fn fetch(&self, binder_id: BinderId) -> crate::Result<PrehniteManaged<Binder>> {
        todo!()
    }
}
