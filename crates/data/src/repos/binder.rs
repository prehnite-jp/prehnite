use crate::database::schema::binder::DbBinder;
use crate::manager::DataManager;
use crate::manager::managed::PrehniteManaged;
use crate::{Binder, BinderId};

pub struct BinderRepository {
    manager: DataManager,
}

impl BinderRepository {
    pub fn new(manager: DataManager) -> Self {
        Self { manager }
    }

    pub async fn create(
        &self,
        title: &str,
        description: Option<&str>,
    ) -> crate::Result<PrehniteManaged<Binder>> {
        let mut conn = self.manager.acquire().await?;
        let db_binder = DbBinder::create(title, description, &mut conn).await?;
        Ok(self.manager.manage(db_binder.into()))
    }

    pub async fn search_by_title(
        &self,
        title: &str,
    ) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        let mut conn = self.manager.acquire().await?;
        Ok(self.convert_binder_from_db_binders(
            DbBinder::search(Some(format!("%{}%", title).as_str()), None, &mut conn).await?,
        ))
    }

    pub async fn search_by_description(
        &self,
        description: &str,
    ) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        let mut conn = self.manager.acquire().await?;
        Ok(self.convert_binder_from_db_binders(
            DbBinder::search(None, Some(format!("%{}%", description).as_str()), &mut conn).await?,
        ))
    }

    pub async fn search(&self, keyword: &str) -> crate::Result<Vec<PrehniteManaged<Binder>>> {
        let keyword = Some(format!("%{}%", keyword));
        let keyword = keyword.as_ref().map(|x| x.as_str());
        let mut conn = self.manager.acquire().await?;
        Ok(self
            .convert_binder_from_db_binders(DbBinder::search(keyword, keyword, &mut conn).await?))
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
