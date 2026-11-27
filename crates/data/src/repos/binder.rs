use crate::Binder;
use crate::database::schema::binder::DbBinder;
use crate::manager::weak_inner_data_manager::WeakInnerDataManager;

#[derive(Clone)]
pub struct BinderRepository {
    manager: WeakInnerDataManager,
}

impl BinderRepository {
    pub(crate) fn new(manager: WeakInnerDataManager) -> Self {
        Self { manager }
    }

    fn convert_binder_from_db_binders(&self, db: Vec<DbBinder>) -> Vec<Binder> {
        db.into_iter().map(Binder::from).collect()
    }
}

#[cfg(not(feature = "gpui"))]
mod _impl {
    use crate::database::schema::binder::DbBinder;
    use crate::manager::PrehniteManaged;
    use crate::manager::weak_inner_data_manager::WeakInnerDataManager;
    use crate::{Binder, BinderId, BinderRepository};

    impl BinderRepository {
        pub async fn create(
            &self,
            title: &str,
            description: Option<&str>,
        ) -> crate::Result<PrehniteManaged<Binder>> {
            let manager = self.manager.get()?;
            let mut conn = manager.acquire().await?;
            let db_binder = DbBinder::create(title, description, &mut conn).await?;
            Ok(self.manager.manage(db_binder.into()))
        }

        pub async fn search_by_title(&self, title: &str) -> crate::Result<Vec<Binder>> {
            let manager = self.manager.get()?;
            let mut conn = manager.acquire().await?;
            Ok(self.convert_binder_from_db_binders(
                DbBinder::search(Some(format!("%{}%", title).as_str()), None, &mut conn).await?,
            ))
        }

        pub async fn search_by_description(&self, description: &str) -> crate::Result<Vec<Binder>> {
            let manager = self.manager.get()?;
            let mut conn = manager.acquire().await?;
            Ok(self.convert_binder_from_db_binders(
                DbBinder::search(None, Some(format!("%{}%", description).as_str()), &mut conn)
                    .await?,
            ))
        }

        pub async fn search(&self, keyword: &str) -> crate::Result<Vec<Binder>> {
            let manager = self.manager.get()?;
            let keyword = Some(format!("%{}%", keyword));
            let keyword = keyword.as_ref().map(|x| x.as_str());
            let mut conn = manager.acquire().await?;
            Ok(self.convert_binder_from_db_binders(
                DbBinder::search(keyword, keyword, &mut conn).await?,
            ))
        }

        pub async fn fetch(
            &self,
            binder_id: BinderId,
        ) -> crate::Result<Option<PrehniteManaged<Binder>>> {
            let manager = self.manager.get()?;
            let mut conn = manager.acquire().await?;
            Ok(DbBinder::fetch(binder_id, &mut conn)
                .await?
                .map(|x| self.manager.manage(x.into())))
        }
    }
}

#[cfg(feature = "gpui")]
mod _gpui_impl {
    use crate::database::schema::binder::DbBinder;
    use crate::manager::PrehniteManaged;
    use crate::{Binder, BinderId, BinderRepository};
    use gpui_kit::{AppContext, Task};

    impl BinderRepository {
        pub fn create<C: AppContext>(
            &self,
            cx: &mut C,
            title: String,
            description: Option<String>,
        ) -> Task<crate::Result<PrehniteManaged<Binder>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let manager = repos.manager.get()?;
                let mut conn = manager.acquire().await?;
                Ok(repos.manager.manage(
                    DbBinder::create(&title, description.as_ref().map(String::as_str), &mut conn)
                        .await?
                        .into(),
                ))
            })
        }

        pub fn search_by_title<C: AppContext>(
            &self,
            cx: &mut C,
            title: String,
        ) -> Task<crate::Result<Vec<Binder>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let manager = repos.manager.get()?;
                let mut conn = manager.acquire().await?;
                Ok(repos.convert_binder_from_db_binders(
                    DbBinder::search(Some(format!("%{}%", title).as_str()), None, &mut conn)
                        .await?,
                ))
            })
        }

        pub fn search_by_description<C: AppContext>(
            &self,
            cx: &mut C,
            description: String,
        ) -> Task<crate::Result<Vec<Binder>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let manager = repos.manager.get()?;
                let mut conn = manager.acquire().await?;
                Ok(repos.convert_binder_from_db_binders(
                    DbBinder::search(None, Some(format!("%{}%", description).as_str()), &mut conn)
                        .await?,
                ))
            })
        }

        pub fn search<C: AppContext>(
            &self,
            cx: &mut C,
            keyword: &str,
        ) -> Task<crate::Result<Vec<Binder>>> {
            let keyword = Some(format!("%{}%", keyword));
            let repos = self.clone();
            cx.background_spawn(async move {
                let manager = repos.manager.get()?;
                let keyword = keyword.as_ref().map(|x| x.as_str());
                let mut conn = manager.acquire().await?;
                Ok(repos.convert_binder_from_db_binders(
                    DbBinder::search(keyword, keyword, &mut conn).await?,
                ))
            })
        }

        pub fn fetch<C: AppContext>(
            &self,
            cx: &mut C,
            binder_id: BinderId,
        ) -> Task<crate::Result<Option<PrehniteManaged<Binder>>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let manager = repos.manager.get()?;
                let mut conn = manager.acquire().await?;
                Ok(DbBinder::fetch(binder_id, &mut conn)
                    .await?
                    .map(|x| repos.manager.manage(x.into())))
            })
        }
    }
}
