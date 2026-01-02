use crate::db::schema::schema_binder_helper_impl::placeholder_in_clause;
use crate::db::schema::{
    BackgroundInfo, BackgroundReference, Bibliography, BibliographyAuthor, Draft, Headline, Item,
    ItemReference, Paragraph, ParagraphSummary, Tag, Task,
};
use sqlx::{Error, Row, SqliteConnection};

impl BackgroundInfo {
    pub async fn load_references(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.references = Some(
            sqlx::query_as::<_, BackgroundReference>(
                "SELECT * FROM view_deserializable_background_reference WHERE background_info_id=?",
            )
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl BackgroundInfo {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_references(conn).await
    }
}

impl Bibliography {
    pub async fn load_authors(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.authors = sqlx::query_as::<_, BibliographyAuthor>(
            "SELECT * FROM main.rel_bibliography_authors
    LEFT OUTER JOIN main.bibliography_authors author
    ON rel_bibliography_authors.bibliography_author_id = author.id
    WHERE rel_bibliography_authors.bibliography_id=?",
        )
            .bind(self.id)
            .fetch_all(conn)
            .await?;
        Ok(())
    }
}

impl Bibliography {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_authors(conn).await
    }
}

impl Item {
    pub async fn load_references(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.references = Some(
            sqlx::query_as::<_, ItemReference>(
                "SELECT * FROM view_deserializable_item_reference WHERE item_id=?",
            )
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_tags(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.tags = Some(
            sqlx::query_as::<_, Tag>("SELECT tags.* FROM tags LEFT OUTER JOIN rel_tag_and_item ON tags.id = rel_tag_and_item.tag_id WHERE item_id=?")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_background_info_list(
        &mut self,
        conn: &mut SqliteConnection,
    ) -> Result<(), Error> {
        self.background_info_list = Some(
            sqlx::query_as::<_, BackgroundInfo>(
                "SELECT background_info.* FROM background_info
    LEFT OUTER JOIN rel_background_and_item
        ON background_info.id = rel_background_and_item.background_info_id
    WHERE item_id=?",
            )
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_tasks(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.tasks = Some(
            sqlx::query_as::<_, Task>("SELECT * FROM view_deserializable_task WHERE item_id=?")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl Item {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_references(conn).await?;
        self.load_tags(conn).await?;
        self.load_background_info_list(conn).await?;
        self.load_tasks(conn).await
    }
}

impl Headline {
    pub async fn load_children(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.children = Some(
            sqlx::query("SELECT id FROM headlines WHERE parent_id = $1")
                .bind(self.id)
                .fetch_all(conn)
                .await?
                .iter()
                .map(|v| v.try_get("id").ok())
                .collect(),
        );
        Ok(())
    }

    pub async fn fetch_children(&self, conn: &mut SqliteConnection) -> Result<Vec<Self>, Error> {
        let id_list: Vec<String> = match self.children.clone() {
            None => {
                vec!["NULL".to_string()]
            }
            Some(v) => v
                .iter()
                .map(|v| match v {
                    None => "NULL".to_string(),
                    Some(v) => v.to_string(),
                })
                .collect(),
        };

        let sql = format!(
            "SELECT * FROM headlines WHERE id IN ({})",
            placeholder_in_clause(&id_list)
        );
        let mut query = sqlx::query_as::<_, Headline>(sql.as_str());
        for i in id_list {
            query = query.bind(i);
        }
        query.fetch_all(conn).await
    }

    pub async fn load_paragraph(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.paragraph = Some(
            sqlx::query_as::<_, Paragraph>("SELECT * FROM item_references WHERE ?")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl Headline {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_children(conn).await?;
        self.load_paragraph(conn).await
    }
}

impl Paragraph {
    pub async fn load_draft(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.draft = Some(
            sqlx::query_as::<_, Draft>("SELECT * FROM draft WHERE paragraph_id=?")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_summary(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.summary = Some(
            sqlx::query_as::<_, ParagraphSummary>(
                "SELECT * FROM paragraph_summaries WHERE paragraph_id=?",
            )
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl Paragraph {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_draft(conn).await?;
        self.load_summary(conn).await
    }
}
