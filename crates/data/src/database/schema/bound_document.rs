use crate::{BinderId, BoundDocumentId, DocumentId};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, FromRow, SqliteConnection, query, query_as};
use std::collections::BTreeMap;

struct BoundDocumentIntoIter {
    ptr_next: Option<BoundDocumentId>,
    inner: BTreeMap<BoundDocumentId, DbBoundDocument>,
}

impl BoundDocumentIntoIter {
    fn new(inner: BTreeMap<BoundDocumentId, DbBoundDocument>) -> Self {
        let ptr_next = inner.values().find(|x| x.prev.is_none());
        let ptr_next = ptr_next.and_then(|x| x.next.as_ref()).copied();
        Self { ptr_next, inner }
    }
}

impl Drop for BoundDocumentIntoIter {
    fn drop(&mut self) {
        if self.ptr_next.is_some() {
            for _ in &mut *self {}
        }
    }
}

impl Iterator for BoundDocumentIntoIter {
    type Item = DbBoundDocument;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.inner.remove(&self.ptr_next?)?;
        self.ptr_next = x.next;
        Some(x)
    }
}

#[derive(FromRow, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct DbBoundDocument {
    pub id: BoundDocumentId,
    pub binder_id: BinderId,
    pub document_id: DocumentId,
    pub next: Option<BoundDocumentId>,
    pub prev: Option<BoundDocumentId>,
}

impl DbBoundDocument {
    pub(crate) async fn add_document(
        binder_id: BinderId,
        document_id: DocumentId,
        next: Option<BoundDocumentId>,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        let mut tx = conn.begin().await?;
        // 挿入する位置の次のページを取得する。
        let next_row = if let Some(x) = next {
            Self::fetch_by_id(x, &mut *tx).await?
        } else {
            // 最終ページを取得
            Self::fetch_next_is_null(binder_id, &mut *tx).await?
        };

        // ページを挿入(c2)
        let res: DbBoundDocument = query_as(
            "INSERT INTO bound_documents(binder_id, document_id, next, prev) VALUES (?,?,?,?) ON CONFLICT DO NOTHING ",
        )
        .bind(binder_id)
        .bind(document_id)
        .bind(next)
        .bind(next_row.prev) // 次のページの前のページを引継ぐ
        .fetch_one(&mut *tx)
        .await?;

        // 前のページにページを登録する。c1
        query("UPDATE bound_documents SET next = ? WHERE id = ?")
            .bind(res.id)
            .bind(res.prev)
            .execute(&mut *tx)
            .await?;

        // 次のページにページを登録する。c1
        query("UPDATE bound_documents SET prev = ? WHERE id = ?")
            .bind(res.id)
            .bind(res.next)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(res)
    }

    /// 指定したドキュメント(new_next)の一つ前にページを移動します。
    pub(crate) async fn move_page(
        self,
        new_next: Option<BoundDocumentId>,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        let mut tx = conn.begin().await?;
        self.remove(&mut *tx).await?;
        let res = Self::add_document(self.binder_id, self.document_id, new_next, &mut *tx).await?;
        tx.commit().await?;
        Ok(res)
    }

    async fn fetch_by_id(id: BoundDocumentId, conn: &mut SqliteConnection) -> crate::Result<Self> {
        Ok(query_as("SELECT * FROM bound_documents WHERE id = ?")
            .bind(id)
            .fetch_one(conn)
            .await?)
    }

    /// バインダーに紐づいているドキュメントをすべて取得します。
    pub(crate) async fn fetch(
        binder_id: BinderId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Vec<DbBoundDocument>> {
        Ok(BoundDocumentIntoIter::new(
            query_as::<_, DbBoundDocument>("SELECT * FROM bound_documents WHERE binder_id = ?")
                .bind(binder_id)
                .fetch_all(conn)
                .await?
                .into_iter()
                .map(|x| (x.id, x))
                .collect(),
        )
        .collect())
    }

    /// ドキュメントをバインダーから取り除きます。
    ///
    /// 存在しない場合、`false` を返します。
    pub(crate) async fn remove(&self, conn: &mut SqliteConnection) -> crate::Result<bool> {
        let mut tx = conn.begin().await?;

        // ドキュメントを削除する。
        if query("DELETE FROM bound_documents WHERE id = ?")
            .bind(self.id)
            .execute(&mut *tx)
            .await?
            .rows_affected()
            <= 0
        {
            tx.rollback().await?;
            return Ok(false);
        }

        // ひとつ前のドキュメントが存在する場合、現在のドキュメントの次をそのドキュメントに設定する。 c1
        query("UPDATE bound_documents SET next = ? WHERE id = ?")
            .bind(self.next)
            .bind(self.prev)
            .execute(&mut *tx)
            .await?;

        // 次のドキュメントが存在する場合、現在のドキュメントの前をそのドキュメントに設定する。 c1
        query("UPDATE bound_documents SET prev = ? WHERE id = ?")
            .bind(self.prev)
            .bind(self.next)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(true)
    }

    async fn fetch_next_is_null(
        binder_id: BinderId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        Ok(
            query_as("SELECT * FROM bound_documents WHERE next IS NULL AND binder_id = ?")
                .bind(binder_id)
                .fetch_one(conn)
                .await?,
        )
    }
}
