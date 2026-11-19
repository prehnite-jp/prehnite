use crate::database::schema::{BinderId, BoundDocumentId, DocumentId};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, FromRow, Row, SqliteConnection, query, query_as};
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
            // 残っている要素を全てドロップします
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

        // 1つ前のドキュメントを取得する。
        let prev = query("SELECT id FROM bound_documents WHERE next = ?")
            .bind(self.id)
            .fetch_optional(&mut *tx)
            .await?
            .map(|x| x.get::<i64, _>(0));

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

        // ひとつ前のドキュメントが存在する場合、現在のドキュメントの次をそのドキュメントに設定する。
        query("UPDATE bound_documents SET next = ? WHERE id = ?")
            .bind(self.next)
            .bind(prev)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(true)
    }
}
