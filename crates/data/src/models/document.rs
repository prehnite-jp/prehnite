use crate::data_options::PrehniteDataOptions;
use crate::database::schema::document::DbDocument;
use crate::database::schema::rel_doc_and_background_info::DbRelDocsAndBackgroundInfo;
use crate::database::schema::rel_doc_and_tag::DbRelDocsAndTags;
use crate::database::zstd_compressed_blob::ZstdCompressedBlob;
use crate::manager::managed::{PrehniteManaged, PrehniteManagedItem};
use crate::{BoxableTantivyTokenizer, DocumentId, TagId};
use jiff_sqlx::ToSqlx;
use prehnite_time_bridge::{TryIntoTantivyDate, jiff};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, SqliteConnection};
use std::ops::Deref;
use tantivy::TantivyDocument;
use tantivy::schema::{FAST, Field, INDEXED, STORED, TEXT, TextFieldIndexing};
// TODO: 起動時に同期処理を実行する。 同期処理は、Tantivy内の最新UPDATED_ATを用い、それ以降に削除・変更・追加がなされたドキュメントを対象とする。

pub(crate) struct DocumentFields;

impl DocumentFields {
    pub const DOCUMENT_ID: Field = Field::from_field_id(0);
    pub const TITLE: Field = Field::from_field_id(1);
    pub const CONTENT: Field = Field::from_field_id(2);
    pub const UPDATED_AT: Field = Field::from_field_id(3);
}

#[derive(Clone, Debug)]
pub(crate) struct DocumentSchema(tantivy::schema::Schema);

impl DocumentSchema {
    pub fn new(options: &PrehniteDataOptions) -> Self {
        let mut builder = tantivy::schema::SchemaBuilder::new();
        let text_options = TEXT.set_indexing_options(
            TextFieldIndexing::default().set_tokenizer(&options.tokenizer.tokenizer_name()),
        ) | STORED;
        builder.add_i64_field("document_id", INDEXED | STORED); //0
        builder.add_text_field("title", text_options.clone()); //1
        builder.add_text_field("content", text_options); //2
        builder.add_date_field("updated_at", FAST | STORED); //3
        DocumentSchema(builder.build())
    }
}

impl Deref for DocumentSchema {
    type Target = tantivy::schema::Schema;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DocumentSchema> for tantivy::schema::Schema {
    fn from(value: DocumentSchema) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Document {
    pub document_id: DocumentId,
    pub title: Option<String>,
    pub content: String,
    pub deleted: bool,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
    pub tags: Vec<TagId>,
    pub background_info: Vec<DocumentId>,
}

impl Document {
    pub(crate) fn from_db(db_document: DbDocument) -> Self {
        Self {
            document_id: db_document.id,
            title: db_document.title.map(|x| x.text().to_string()),
            content: db_document.content.text().to_string(),
            deleted: db_document.deleted,
            created_at: db_document.created_at.to_jiff(),
            updated_at: db_document.updated_at.to_jiff(),
            tags: vec![],
            background_info: vec![],
        }
    }

    pub(crate) fn try_into_tantivy(self) -> crate::Result<TantivyDocument> {
        let mut doc = TantivyDocument::new();
        doc.add_i64(DocumentFields::DOCUMENT_ID, *self.document_id);
        doc.add_text(DocumentFields::TITLE, self.title.unwrap_or_default());
        doc.add_text(DocumentFields::CONTENT, self.content);
        doc.add_date(
            DocumentFields::UPDATED_AT,
            self.updated_at.try_into_tantivy_date()?,
        );
        Ok(doc)
    }
}

#[derive(Debug)]
pub enum DeleteStatus {
    Trashed,
    Deleted,
    NotFound,
}

impl PrehniteManaged<Document> {
    pub async fn delete(&mut self) -> crate::Result<DeleteStatus> {
        self.manager()
            .get_manager()?
            .document_index
            .delete(self.document_id)?;
        let mut conn = self.manager().acquire().await?;
        let mut tx = conn.begin().await?;
        if self.deleted {
            if DbDocument::delete(self.document_id, &mut *tx).await? {
                tx.commit().await?;
                Ok(DeleteStatus::Deleted)
            } else {
                Ok(DeleteStatus::NotFound)
            }
        } else {
            self.as_mut().deleted = true;
            if self.save(&mut *tx).await? {
                tx.commit().await?;
                Ok(DeleteStatus::Trashed)
            } else {
                Ok(DeleteStatus::NotFound)
            }
        }
    }

    pub async fn restore(&mut self) -> crate::Result<()> {
        if self.deleted {
            self.as_mut().deleted = false;
            let mut conn = self.manager().acquire().await?;
            self.save(&mut *conn).await?;
        }
        Ok(())
    }

    pub async fn update_title(&mut self, title: Option<impl Into<String>>) -> crate::Result<()> {
        let mut conn = self.manager().acquire().await?;
        self.as_mut().title = title.map(Into::into);
        self.save(&mut *conn).await?;
        Ok(())
    }

    pub async fn update_content(mut self, content: impl Into<String>) -> crate::Result<()> {
        let mut conn = self.manager().acquire().await?;
        self.as_mut().content = content.into();
        self.save(&mut *conn).await?;
        Ok(())
    }

    async fn save(&mut self, conn: &mut SqliteConnection) -> crate::Result<bool> {
        Ok(DbDocument::from(self.clone()).save(conn).await?)
    }

    pub async fn add_tag(&mut self, tag_id: TagId) -> crate::Result<()> {
        let mut conn = self.manager().acquire().await?;
        if DbRelDocsAndTags::create(self.document_id, tag_id, &mut *conn)
            .await?
            .is_some()
        {
            self.as_mut().tags.push(tag_id);
        }
        Ok(())
    }

    pub async fn remove_tag(&mut self, tag_id: TagId) -> crate::Result<()> {
        let mut conn = self.manager().acquire().await?;
        DbRelDocsAndTags::delete(self.document_id, tag_id, &mut *conn).await?;
        self.as_mut().tags.retain(|x| *x != tag_id);
        Ok(())
    }

    pub async fn add_background_info(
        &mut self,
        bg_info_document_id: DocumentId,
    ) -> crate::Result<()> {
        let mut conn = self.manager().acquire().await?;
        if DbRelDocsAndBackgroundInfo::create(self.document_id, bg_info_document_id, &mut *conn)
            .await?
            .is_some()
        {
            self.as_mut().background_info.push(bg_info_document_id);
        }
        Ok(())
    }

    pub async fn remove_background_info(
        &mut self,
        bg_info_document_id: DocumentId,
    ) -> crate::Result<()> {
        let mut conn = self.manager().acquire().await?;
        DbRelDocsAndBackgroundInfo::delete(self.document_id, bg_info_document_id, &mut *conn)
            .await?;
        self.as_mut()
            .background_info
            .retain(|x| *x != bg_info_document_id);
        Ok(())
    }
}

impl PrehniteManagedItem for Document {}

impl From<Document> for DbDocument {
    fn from(value: Document) -> Self {
        let Document {
            document_id: id,
            title,
            content,
            deleted,
            created_at,
            updated_at,
            ..
        } = value;
        DbDocument {
            id,
            title: title.map(|title| ZstdCompressedBlob::from(title.as_bytes().to_vec())),
            content: ZstdCompressedBlob::from(content.as_bytes().to_vec()),
            deleted,
            created_at: created_at.to_sqlx(),
            updated_at: updated_at.to_sqlx(),
        }
    }
}

impl From<DbDocument> for Document {
    fn from(value: DbDocument) -> Self {
        let DbDocument {
            id: document_id,
            title,
            content,
            deleted,
            created_at,
            updated_at,
        } = value;
        let title = title.map(|x| x.text().to_string());
        let content = content.text().to_string();
        let created_at = created_at.to_jiff();
        let updated_at = updated_at.to_jiff();
        Self {
            document_id,
            title,
            content,
            deleted,
            created_at,
            updated_at,
            tags: vec![],
            background_info: vec![],
        }
    }
}
