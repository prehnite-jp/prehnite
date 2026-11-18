use crate::binder::DbBinder;
use crate::manager::managed::{PrehniteManaged, PrehniteManagedItem};
use crate::{BinderId, DocumentId};
use prehnite_time_bridge::jiff;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Binder {
    pub id: BinderId,
    pub title: String,
    pub details: Option<String>,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

impl PrehniteManaged<Binder> {
    pub async fn delete(self) -> crate::Result<bool> {
        let conn = &mut *self.manager().acquire().await?;
        DbBinder::from(self.item()).delete(conn).await
    }

    pub async fn update_title(self, title: String) -> crate::Result<Self> {
        todo!()
    }

    pub async fn update_details(self, details: Option<String>) -> crate::Result<Self> {
        todo!()
    }

    /// ページを指定された場所に新しく追加します。既にドキュメントがページとして存在している場合は何もしません。
    pub async fn add_document(
        self,
        document_id: DocumentId,
        pos: BinderPagePosition,
    ) -> crate::Result<Self> {
        todo!()
    }

    /// 指定されたページを、posに移動します。ページが存在しない場合は何もしません。
    pub async fn move_document(self, page: u64, pos: BinderPagePosition) -> crate::Result<Self> {
        todo!()
    }

    /// 指定されたドキュメントをバインダーから取り除きます。ドキュメントがページとして存在しない場合は何もしません。
    pub async fn remove_document(self, document_id: DocumentId) -> crate::Result<Self> {
        if let Some(page) = self.find_page(document_id)? {
            self.remove_page(page).await
        } else {
            Ok(self)
        }
    }

    /// 指定されたページをバインダーから取り除きます。ページが存在しない場合は何もしません。
    ///
    /// 例: 0A,1B,2C,3D|(idxName) ここからidx2を取り除いた場合、ページは次のようになります。 0A,1B,2D
    pub async fn remove_page(self, page: u64) -> crate::Result<Self> {
        todo!()
    }

    /// ドキュメントのページ番号を探索します。
    pub fn find_page(&self, document_id: DocumentId) -> crate::Result<Option<u64>> {
        todo!()
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
