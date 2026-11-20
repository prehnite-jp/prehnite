use crate::{Document, DocumentId, DocumentSchema};
use crate::data_options::PrehniteDataOptions;
use debug_ignore::DebugIgnore;
use sqlx::SqliteConnection;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use tantivy::directory::MmapDirectory;
use tantivy::query::{Query, QueryParser};
use tantivy::query_grammar::{UserInputAst, UserInputLeaf, parse_query};
use tantivy::{Index, IndexWriter, Term};
use tokio::fs::create_dir_all;
use crate::database::schema::metadata::{DbMetadata, DbMetadataKey};
use crate::database::schema::rel_doc_and_tag::DbRelDocsAndTags;
use crate::models::DocumentFields;

#[allow(unused)]
#[derive(Debug)]
pub struct DocumentIndex {
    pub(crate) assigned_dir: PathBuf,
    pub(crate) index: Index,
    pub(crate) writer: DebugIgnore<Mutex<IndexWriter>>,
    pub(crate) parser: DebugIgnore<QueryParser>,
}

impl Deref for DocumentIndex {
    type Target = Index;

    fn deref(&self) -> &Self::Target {
        &self.index
    }
}

impl DocumentIndex {
    pub(crate) async fn open_or_create(
        data_options: &PrehniteDataOptions,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Self> {
        let prehnite_id = DbMetadata::fetch(DbMetadataKey::PrehniteId, conn).await?;
        let assigned_dir = data_options
            .index_cache_base_dir
            .join(prehnite_id.value.ok_or(crate::Error::PrehniteIdMissing)?);
        create_dir_all(&assigned_dir).await?;

        let schema = DocumentSchema::new(data_options);
        let index =
            Index::open_or_create(MmapDirectory::open(&assigned_dir)?, schema.clone().into())?;
        let writer = Mutex::new(index.writer(100_000_000)?).into();
        let parser = DebugIgnore(QueryParser::for_index(
            &index,
            vec![DocumentFields::TITLE, DocumentFields::CONTENT],
        ));
        Ok(Self {
            assigned_dir,
            index,
            writer: DebugIgnore(writer),
            parser,
        })
    }

    fn writer(&self) -> MutexGuard<'_, IndexWriter> {
        self.writer.lock().unwrap()
    }

    pub(crate) fn delete(&self, document_id: DocumentId) -> crate::Result<()> {
        let mut w = self.writer();
        w.delete_term(Term::from_field_i64(
            DocumentFields::DOCUMENT_ID,
            *document_id,
        ));
        w.commit()?;
        Ok(())
    }

    pub(crate) fn upsert(&self, document: Document) -> crate::Result<()> {
        let mut w = self.writer();
        w.delete_term(Term::from_field_i64(
            DocumentFields::DOCUMENT_ID,
            *document.document_id,
        ));
        w.add_document(document.try_into_tantivy()?)?;
        w.commit()?;
        Ok(())
    }

    pub(crate) fn parse_query(&self, query: &str) -> crate::Result<Box<dyn Query>> {
        Ok(self.parser.parse_query(query)?)
    }

    pub(crate) async fn parse_query_with_tags(
        &self,
        query: &str,
        tags: Vec<String>,
        conn: &mut SqliteConnection,
    ) -> crate::Result<Box<dyn Query>> {
        let parsed = parse_query(&query)
            .map_err(|e| crate::Error::TantivyQueryParseErrorQueryGrammar(format!("{:?}", e)))?;
        let tag_filter_query = UserInputAst::Leaf(Box::new(UserInputLeaf::Set {
            field: Some("document_id".into()),
            elements: DbRelDocsAndTags::fetch_docs(tags, conn)
                .await?
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
        }));
        let parsed = UserInputAst::and(vec![tag_filter_query, parsed]);

        Ok(self.parser.build_query_from_user_input_ast(parsed)?)
    }
}
