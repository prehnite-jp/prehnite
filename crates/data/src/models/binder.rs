use crate::database::schema::binder::DbBinder;
use crate::database::schema::bound_document::DbBoundDocument;
use crate::manager::managed::{PrehniteManaged, PrehniteManagedItem};
use crate::{BinderId, BoundDocumentId, DocumentId};
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
    pub title: String,
    pub details: Option<String>,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
    pub pages: Vec<DocumentId>,
    db_pages: Vec<DbBoundDocument>,
}

impl Binder {
    pub(crate) fn sync_pages(&mut self) {
        self.pages = self.db_pages.iter().map(|x| x.document_id).collect();
    }
}

impl PrehniteManaged<Binder> {
    pub async fn delete(&mut self) -> crate::Result<bool> {
        let conn = &mut *self.manager().acquire().await?;
        DbBinder::from(self.clone()).delete(conn).await
    }

    pub async fn update_title(&mut self, title: String) -> crate::Result<()> {
        self.as_mut().title = title;
        let conn = &mut self.manager().acquire().await?;
        DbBinder::from(self.clone()).save(conn).await?;
        Ok(())
    }

    pub async fn update_details(&mut self, details: Option<String>) -> crate::Result<()> {
        self.as_mut().details = details;
        let conn = &mut self.manager().acquire().await?;
        DbBinder::from(self.clone()).save(conn).await?;
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
            &mut *conn,
        )
        .await?;
        self.as_mut().db_pages = DbBoundDocument::fetch(self.id, &mut *conn).await?;
        self.as_mut().sync_pages();
        Ok(())
    }

    /// 指定されたページを、posに移動します。ページが存在しない場合は何もしません。
    pub async fn move_document(&mut self, page: u64, pos: BinderPagePosition) -> crate::Result<()> {
        if (page as usize) < self.db_pages.len() {
            let page = self.as_mut().db_pages.remove(page as usize);
            let mut conn = self.manager().acquire().await?;
            let pos = pos.to_new_next(&self.db_pages);
            page.move_page(pos, &mut *conn).await?;
            self.as_mut().db_pages = DbBoundDocument::fetch(self.id, &mut *conn).await?;
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
            let conn = &mut *self.manager().acquire().await?;
            let page = self.as_mut().db_pages.remove(page as usize);
            page.remove(conn).await?;
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
        DbBinder {
            id,
            title,
            details,
            created_at,
            updated_at,
        }
    }
}
