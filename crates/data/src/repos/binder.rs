use crate::BinderId;
use crate::binder::DbBinder;
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
        let mut conn = self.manager.acquire().await?;
        let db_binder = DbBinder::create(title, description, &mut conn).await?;
        Ok(self.manager.manage(db_binder.into()))
    }

    pub async fn search_by_title(
        &self,
        title: String,
    ) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        let mut conn = self.manager.acquire().await?;
        Ok(self.convert_binder_from_db_binders(
            DbBinder::search(Some(format!("%{}%", title)), None, &mut conn).await?,
        ))
    }

    pub async fn search_by_description(
        &self,
        description: String,
    ) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        let mut conn = self.manager.acquire().await?;
        Ok(self.convert_binder_from_db_binders(
            DbBinder::search(None, Some(format!("%{}%", description)), &mut conn).await?,
        ))
    }

    pub async fn search(&self, keyword: String) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        let keyword = Some(format!("%{}%", keyword));
        let mut conn = self.manager.acquire().await?;
        Ok(self.convert_binder_from_db_binders(
            DbBinder::search(keyword.clone(), keyword, &mut conn).await?,
        ))
    }

    pub async fn fetch(
        &self,
        binder_id: BinderId,
    ) -> crate::Result<Option<PrehniteManaged<Binder>>> {
        let mut conn = self.manager.acquire().await?;
        Ok(DbBinder::fetch(binder_id, &mut conn)
            .await?
            .map(|x| self.manager.manage(x.into())))
    }

    fn convert_binder_from_db_binders(&self, db: Vec<DbBinder>) -> Vec<PrehniteManaged<Binder>> {
        db.into_iter()
            .map(|x| self.manager.manage(x.into()))
            .collect()
    }
}
