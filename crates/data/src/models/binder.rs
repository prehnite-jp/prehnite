use crate::database::schema::binder::DbBinder;
use crate::database::schema::bound_document::DbBoundDocument;
use crate::manager::managed::{PrehniteManaged, PrehniteManagedItem};
use crate::{BinderId, BoundDocumentId, DocumentId, TString};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum BinderPagePosition {
    /// 最初にページを挿入します。
    ///
    /// 例:
    ///
    /// A,B,C,D へ E を挿入した場合、結果は次のようになります。
    ///
    /// E,A,B,C,D
    First,
    /// 指定したページ番号にページを挿入します。ページは0-based indexです。
    ///
    /// 例:
    ///
    /// A,B,C,D へ E を2ページ目に挿入したとき、結果は次のようになります。
    ///
    /// |0|1|2|3|4|
    /// |--|--|--|--|--|
    /// |A|B|E|C|D|
    ///
    /// つまり、指定したページにドキュメントを挿入し、シフトします。
    InsertAndShiftPage(u64),
    /// 最後にページを挿入します。
    ///
    /// 例:
    ///
    /// A,B,C,D へ E を挿入した場合、結果は次のようになります。
    ///
    /// A,B,C,D,E
    Back,
}

impl BinderPagePosition {
    fn to_new_next(self, db_pages: &Vec<DbBoundDocument>) -> Option<BoundDocumentId> {
        match self {
            BinderPagePosition::First => Some(db_pages.first()?.id),
            BinderPagePosition::InsertAndShiftPage(x) => {
                let v = db_pages.get(x as usize)?;
                Some(v.id)
            }
            BinderPagePosition::Back => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Binder {
    pub id: BinderId,
    pub title: TString,
    pub details: Option<TString>,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
    pub pages: Vec<DocumentId>,
    #[serde(skip)]
    db_pages: Vec<DbBoundDocument>,
}

impl Binder {
    pub(crate) fn sync_pages(&mut self) {
        self.pages = self.db_pages.iter().map(|x| x.document_id).collect();
    }
}

impl PrehniteManaged<Binder> {
    pub async fn delete(&mut self) -> crate::Result<bool> {
        DbBinder::from(self.clone_item())
            .delete(&mut *self.manager().acquire().await?)
            .await
    }

    pub async fn update_title(&mut self, title: &str) -> crate::Result<()> {
        self.as_mut().title = title.into();
        DbBinder::from(self.clone_item())
            .save(&mut *self.manager().acquire().await?)
            .await?;
        Ok(())
    }

    pub async fn update_details(&mut self, details: Option<&str>) -> crate::Result<()> {
        self.as_mut().details = details.map(Into::into);
        DbBinder::from(self.clone_item())
            .save(&mut *self.manager().acquire().await?)
            .await?;
        Ok(())
    }

    /// ページを指定された場所に新しく追加します。既にドキュメントがページとして存在している場合はエラーが発生します。
    pub async fn add_document(
        &mut self,
        document_id: DocumentId,
        pos: BinderPagePosition,
    ) -> crate::Result<()> {
        let mut conn = self.manager().acquire().await?;
        DbBoundDocument::add_document(
            self.id,
            document_id,
            pos.to_new_next(&self.db_pages),
            &mut conn,
        )
        .await?;
        self.as_mut().db_pages = DbBoundDocument::fetch(self.id, &mut conn).await?;
        self.as_mut().sync_pages();
        Ok(())
    }

    /// 指定されたページを、posに移動します。ページが存在しない場合は何もしません。
    pub async fn move_document(&mut self, page: u64, pos: BinderPagePosition) -> crate::Result<()> {
        if (page as usize) < self.db_pages.len() {
            let page = self.as_mut().db_pages.remove(page as usize);
            let mut conn = self.manager().acquire().await?;
            let pos = pos.to_new_next(&self.db_pages);
            page.move_page(pos, &mut conn).await?;
            self.as_mut().db_pages = DbBoundDocument::fetch(self.id, &mut conn).await?;
            self.as_mut().sync_pages();
        }
        Ok(())
    }

    /// 指定されたドキュメントをバインダーから取り除きます。ドキュメントがページとして存在しない場合は何もしません。
    pub async fn remove_document(&mut self, document_id: DocumentId) -> crate::Result<()> {
        if let Some(page) = self.find_page(document_id) {
            self.remove_page(page).await?;
        }
        Ok(())
    }

    /// 指定されたページをバインダーから取り除きます。ページが存在しない場合は何もしません。
    ///
    /// 例: 0A,1B,2C,3D|(idxName) ここからidx2を取り除いた場合、ページは次のようになります。 0A,1B,2D
    pub async fn remove_page(&mut self, page: u64) -> crate::Result<()> {
        if page < self.pages.len() as u64 {
            let mut conn = self.manager().acquire().await?;
            let page = self.as_mut().db_pages.remove(page as usize);
            page.remove(&mut conn).await?;
            self.as_mut().sync_pages();
        }
        Ok(())
    }

    /// ドキュメントのページ番号を探索します。
    pub fn find_page(&self, document_id: DocumentId) -> Option<u64> {
        self.pages
            .iter()
            .enumerate()
            .find(|(_, doc)| **doc == document_id)
            .map(|x| x.0 as u64)
    }
}

impl PrehniteManagedItem for Binder {}

impl From<DbBinder> for Binder {
    fn from(value: DbBinder) -> Self {
        let DbBinder {
            id,
            title,
            details,
            created_at,
            updated_at,
        } = value;
        let created_at = created_at.to_jiff();
        let updated_at = updated_at.to_jiff();
        let title = title.into();
        let details = details.map(Into::into);
        Binder {
            id,
            title,
            details,
            created_at,
            updated_at,
            pages: vec![],
            db_pages: vec![],
        }
    }
}
impl From<Binder> for DbBinder {
    fn from(value: Binder) -> Self {
        let Binder {
            id,
            title,
            details,
            created_at,
            updated_at,
            ..
        } = value;
        let created_at = created_at.into();
        let updated_at = updated_at.into();
        let title = title.into();
        let details = details.map(Into::into);
        DbBinder {
            id,
            title,
            details,
            created_at,
            updated_at,
        }
    }
}

#[cfg(feature = "gpui")]
mod _gpui_impl {
    use crate::Result;
    use crate::database::schema::binder::DbBinder;
    use crate::database::schema::bound_document::DbBoundDocument;
    use crate::manager::PrehniteManagedEntity;
    use crate::{Binder, BinderPagePosition, DocumentId};
    use gpui_kit::{App, AppContext, Task};

    impl PrehniteManagedEntity<Binder> {
        pub fn delete(&self, cx: &mut App) -> Task<Result<bool>> {
            let entity = self.clone();
            cx.spawn(async move |cx| {
                let item = entity.clone_inner(cx);
                let res = cx
                    .background_spawn(async move {
                        let mut conn = item.manager().acquire().await?;
                        DbBinder::from(item.unmanage()).delete(&mut conn).await
                    })
                    .await?;
                entity.set_deleted(cx, true);
                Ok(res)
            })
        }

        pub fn update_title(&self, cx: &mut App, title: &str) -> Task<Result<()>> {
            let entity = self.clone();
            let title = title.into();
            cx.spawn(async move |cx| {
                let mut managed = entity.clone_inner(cx);
                managed.as_mut().title = title;
                let item = managed.clone_item();
                let mut conn = managed.manager().acquire().await?;
                cx.background_spawn(async move { DbBinder::from(item).save(&mut *conn).await })
                    .await?;
                entity.update(cx, move |item, cx| {
                    *item = managed;
                    cx.notify();
                });
                Ok(())
            })
        }

        pub fn update_details(&self, cx: &mut App, details: Option<&str>) -> Task<Result<()>> {
            let entity = self.clone();
            let details = details.map(Into::into);
            cx.spawn(async move |cx| {
                let mut managed = entity.clone_inner(cx);
                managed.as_mut().details = details;
                let item = managed.clone_item();
                let mut conn = managed.manager().acquire().await?;
                cx.background_spawn(async move { DbBinder::from(item).save(&mut *conn).await })
                    .await?;
                entity.update(cx, move |item, cx| {
                    *item = managed;
                    cx.notify();
                });
                Ok(())
            })
        }

        /// ページを指定された場所に新しく追加します。既にドキュメントがページとして存在している場合はエラーが発生します。
        pub fn add_document(
            &self,
            cx: &mut App,
            document_id: DocumentId,
            pos: BinderPagePosition,
        ) -> Task<Result<()>> {
            let entity = self.clone();
            cx.spawn(async move |cx| {
                let mut conn = entity.manager(cx).acquire().await?;
                let managed = entity.clone_inner(cx);
                let pages = cx
                    .background_spawn(async move {
                        DbBoundDocument::add_document(
                            managed.id,
                            document_id,
                            pos.to_new_next(&managed.db_pages),
                            &mut conn,
                        )
                        .await?;
                        DbBoundDocument::fetch(managed.id, &mut conn).await
                    })
                    .await?;
                entity.update(cx, |v, cx| {
                    v.as_mut().db_pages = pages;
                    v.as_mut().sync_pages();
                    cx.notify()
                });

                Ok(())
            })
        }

        /// 指定されたページを、posに移動します。ページが存在しない場合は何もしません。
        pub fn move_document(
            &self,
            cx: &mut App,
            page: u64,
            pos: BinderPagePosition,
        ) -> Task<Result<()>> {
            let entity = self.clone();
            cx.spawn(async move |cx| {
                let mut managed = entity.clone_inner(cx);
                if (page as usize) < managed.db_pages.len() {
                    let page = managed.as_mut().db_pages.remove(page as usize);
                    let mut conn = managed.manager().acquire().await?;
                    let pos = pos.to_new_next(&managed.db_pages);
                    page.move_page(pos, &mut conn).await?;
                    let binder_id = managed.id;
                    managed.as_mut().db_pages = cx
                        .background_spawn(async move {
                            DbBoundDocument::fetch(binder_id, &mut conn).await
                        })
                        .await?;
                    managed.as_mut().sync_pages();
                    entity.update(cx, |e, cx| {
                        *e = managed;
                        cx.notify();
                    });
                }
                Ok(())
            })
        }

        /// 指定されたドキュメントをバインダーから取り除きます。ドキュメントがページとして存在しない場合は何もしません。
        pub fn remove_document(&self, cx: &mut App, document_id: DocumentId) -> Task<Result<()>> {
            if let Some(page) = self.find_page(cx, document_id) {
                self.remove_page(cx, page)
            } else {
                Task::ready(Ok(()))
            }
        }

        /// 指定されたページをバインダーから取り除きます。ページが存在しない場合は何もしません。
        ///
        /// 例: 0A,1B,2C,3D|(idxName) ここからidx2を取り除いた場合、ページは次のようになります。 0A,1B,2D
        pub fn remove_page(&self, cx: &mut App, page: u64) -> Task<Result<()>> {
            let entity = self.clone();
            cx.spawn(async move |cx| {
                let mut managed = entity.clone_inner(cx);
                if page < managed.pages.len() as u64 {
                    let mut conn = managed.manager().acquire().await?;
                    let page = managed.as_mut().db_pages.remove(page as usize);
                    cx.background_spawn(async move { page.remove(&mut conn).await })
                        .await?;
                    managed.as_mut().sync_pages();
                    entity.update(cx, move |v, cx| {
                        *v = managed;
                        cx.notify();
                    });
                }
                Ok(())
            })
        }

        /// ドキュメントのページ番号を探索します。
        pub fn find_page(&self, cx: &mut App, document_id: DocumentId) -> Option<u64> {
            self.read(cx)
                .pages
                .iter()
                .enumerate()
                .find(|(_, doc)| **doc == document_id)
                .map(|x| x.0 as u64)
        }
    }
}
