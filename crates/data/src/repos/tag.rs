use crate::database::schema::tag::DbTag;
use crate::manager::DataManager;
use crate::manager::managed::PrehniteManaged;
use crate::{Tag, TagId, TagMetadata};
use sqlx::Acquire;

pub struct TagRepository {
    manager: DataManager,
}

impl TagRepository {
    pub fn new(manager: DataManager) -> Self {
        Self { manager }
    }

    pub async fn create(
        &self,
        title: String,
        details: Option<String>,
        metadata: Option<TagMetadata>,
    ) -> crate::Result<PrehniteManaged<Tag>> {
        let mut conn = self.manager.acquire().await?;
        let mut tx = conn.begin().await?;
        let res = DbTag::fetch_or_create(title, details, metadata, &mut *tx).await?;
        tx.commit().await?;
        Ok(self.manager.manage(Tag::from(res)))
    }

    pub async fn list(&self) -> crate::Result<Vec<PrehniteManaged<Tag>>> {
        let mut conn = self.manager.acquire().await?;
        let res = DbTag::fetch_all(&mut *conn).await?;
        Ok(res
            .into_iter()
            .map(|x| self.manager.manage(x.into()))
            .collect())
    }

    pub async fn fetch_by_id(&self, tag_id: TagId) -> crate::Result<PrehniteManaged<Tag>> {
        let mut conn = self.manager.acquire().await?;
        Ok(self.manager.manage(
            DbTag::fetch_by_id(tag_id, &mut *conn)
                .await?
                .ok_or(crate::Error::MissingTag(tag_id.to_string()))?
                .into(),
        ))
    }

    pub async fn fetch_by_title(&self, title: String) -> crate::Result<PrehniteManaged<Tag>> {
        let mut conn = self.manager.acquire().await?;
        Ok(self.manager.manage(
            DbTag::fetch(&title, &mut *conn)
                .await?
                .ok_or(crate::Error::MissingTag(title))?
                .into(),
        ))
    }
}
