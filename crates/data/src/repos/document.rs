use crate::database::schema::document::DbDocument;
use crate::database::schema::rel_doc_and_background_info::DbRelDocsAndBackgroundInfo;
use crate::database::schema::rel_doc_and_tag::DbRelDocsAndTags;
use crate::manager::weak_inner_data_manager::WeakInnerDataManager;
use crate::{Document, DocumentId};
use sqlx::SqliteConnection;

#[derive(Clone)]
pub struct DocumentRepository {
    manager: WeakInnerDataManager,
}

const DEFAULT_PAGE_SIZE: u64 = 50;

impl DocumentRepository {
    pub(crate) fn new(manager: WeakInnerDataManager) -> Self {
        Self { manager }
    }

    async fn fetch_inner(
        &self,
        document_id: DocumentId,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Document> {
        let docs = DbDocument::fetch(document_id, conn).await?;
        let mut docs: Document = docs.into();
        docs.tags = DbRelDocsAndTags::fetch_tags(document_id, conn).await?;
        docs.background_info =
            DbRelDocsAndBackgroundInfo::fetch_background_info(document_id, conn).await?;
        Ok(docs)
    }
}

#[cfg(not(feature = "gpui"))]
mod _impl {
    use crate::database::schema::document::DbDocument;
    use crate::database::schema::rel_doc_and_background_info::DbRelDocsAndBackgroundInfo;
    use crate::database::schema::rel_doc_and_tag::DbRelDocsAndTags;
    use crate::manager::PrehniteManaged;
    use crate::models::DocumentFields;
    use crate::repos::document::DEFAULT_PAGE_SIZE;
    use crate::{Document, DocumentId, DocumentRepository};
    use sqlx::{Acquire, SqliteConnection};
    use tantivy::TantivyDocument;
    use tantivy::collector::TopDocs;
    use tantivy::schema::Value;

    impl DocumentRepository {
        // TODO:(仮実装) クエリの発行が雑なため、要最適化
        pub async fn search(
            &self,
            page: Option<u64>,
            page_size: Option<u64>,
            query: &str,
            tags: Vec<&str>, // タグはAND条件
        ) -> crate::Result<Vec<Document>> {
            let manager = self.manager.get()?;
            let reader = manager.document_index.reader()?;
            let searcher = reader.searcher();
            let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);
            let offset = (page.unwrap_or_default().max(1) - 1) * page_size;

            let mut conn = manager.acquire().await?;
            let mut tx = conn.begin().await?;
            let query = manager
                .document_index
                .parse_query_with_tags(query, tags, &mut *tx)
                .await?;
            // TODO: ソート方法は検討中
            let docs = searcher.search(
                &query,
                &TopDocs::with_limit(page_size as usize)
                    .and_offset(offset as usize)
                    .order_by_score(),
            )?;

            let mut result = vec![];
            for (_score, doc) in docs {
                let x = searcher.doc::<TantivyDocument>(doc)?;
                let document_id = DocumentId::new(
                    x.get_first(DocumentFields::DOCUMENT_ID)
                        .unwrap()
                        .as_i64()
                        .unwrap(),
                );
                result.push(self.fetch_inner(document_id, &mut *tx).await?);
            }
            Ok(result)
        }

        pub async fn fetch(
            &self,
            document_id: DocumentId,
        ) -> crate::Result<PrehniteManaged<Document>> {
            let mgr = self.manager.get()?;
            let mut conn = mgr.acquire().await?;
            let mut tx = conn.begin().await?;
            Ok(self
                .manager
                .manage(self.fetch_inner(document_id, &mut *tx).await?))
        }

        pub async fn create(
            &self,
            title: &str,
            content: &str,
        ) -> crate::Result<PrehniteManaged<Document>> {
            let manager = self.manager.get()?;
            let mut writer = manager.document_index.writer.lock().unwrap();
            let mut conn = manager.acquire().await?;
            let mut tx = conn.begin().await?;
            let document = Document::from_db(DbDocument::create(&mut *tx, title, content).await?);
            writer.add_document(document.clone().try_into_tantivy()?)?;
            writer.commit()?;
            tx.commit().await?;
            Ok(self.manager.manage(document))
        }
    }
}

#[cfg(feature = "gpui")]
mod _gpui_impl {
    use crate::database::schema::document::DbDocument;
    use crate::manager::PrehniteManaged;
    use crate::models::DocumentFields;
    use crate::repos::document::DEFAULT_PAGE_SIZE;
    use crate::{Document, DocumentId, DocumentRepository};
    use gpui_kit::{App, AppContext, Task};
    use sqlx::Acquire;
    use tantivy::TantivyDocument;
    use tantivy::collector::TopDocs;
    use tantivy::schema::Value;

    impl DocumentRepository {
        // TODO:(仮実装) クエリの発行が雑なため、要最適化
        pub fn search<C: AppContext>(
            &self,
            cx: &mut C,
            page: Option<u64>,
            page_size: Option<u64>,
            query: impl AsRef<str> + Send + Sync + 'static,
            tags: Vec<impl AsRef<str> + Send + Sync + 'static>, // タグはAND条件
        ) -> Task<crate::Result<Vec<Document>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let manager = repos.manager.get()?;
                let reader = manager.document_index.reader()?;
                let searcher = reader.searcher();
                let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);
                let offset = (page.unwrap_or_default().max(1) - 1) * page_size;

                let mut conn = manager.acquire().await?;
                let mut tx = conn.begin().await?;
                let query = manager
                    .document_index
                    .parse_query_with_tags(query, tags, &mut *tx)
                    .await?;
                // TODO: ソート方法は検討中
                let docs = searcher.search(
                    &query,
                    &TopDocs::with_limit(page_size as usize)
                        .and_offset(offset as usize)
                        .order_by_score(),
                )?;

                let mut result = vec![];
                for (_score, doc) in docs {
                    let x = searcher.doc::<TantivyDocument>(doc)?;
                    let document_id = DocumentId::new(
                        x.get_first(DocumentFields::DOCUMENT_ID)
                            .unwrap()
                            .as_i64()
                            .unwrap(),
                    );
                    result.push(repos.fetch_inner(document_id, &mut *tx).await?);
                }
                Ok(result)
            })
        }

        pub fn fetch<C: AppContext>(
            &self,
            cx: &mut C,
            document_id: DocumentId,
        ) -> Task<crate::Result<PrehniteManaged<Document>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let mgr = repos.manager.get()?;
                let mut conn = mgr.acquire().await?;
                let mut tx = conn.begin().await?;
                Ok(repos
                    .manager
                    .manage(repos.fetch_inner(document_id, &mut *tx).await?))
            })
        }

        pub fn create(
            &self,
            cx: &mut App,
            title: String,
            content: String,
        ) -> Task<crate::Result<PrehniteManaged<Document>>> {
            let repos = self.clone();
            cx.spawn(async move |cx| {
                let manager = repos.manager.get()?;
                let manager2 = manager.clone();
                let mut writer = manager.document_index.writer.lock().unwrap();
                let document = cx
                    .background_spawn(async move {
                        let mut conn = manager2.acquire().await?;
                        let mut tx = conn.begin().await?;
                        let document = Document::from_db(
                            DbDocument::create(&mut *tx, &title, &content).await?,
                        );
                        tx.commit().await?;
                        Ok::<_, crate::Error>(document)
                    })
                    .await?;
                writer.add_document(document.clone().try_into_tantivy()?)?;
                writer.commit()?;
                Ok(repos.manager.manage(document))
            })
        }
    }
}
