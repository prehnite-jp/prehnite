use crate::db::query::{
    FETCH_BACKGROUND_INFO_FROM_ITEM_ID_SQL, FETCH_BACKGROUND_REFERENCES_SQL,
    FETCH_BIBLIOGRAPHY_AUTHORS_SQL, FETCH_HEADLINE_CHILDREN_RECURSE_SQL,
    FETCH_HEADLINE_RELATED_PARAGRAPH_SQL, FETCH_ITEM_REFERENCES_SQL, FETCH_ITEM_RELATED_TAGS_SQL,
    FETCH_ITEM_RELATED_TASKS_SQL, FETCH_PARAGRAPH_RELATED_DRAFT_SQL,
    FETCH_PARAGRAPH_RELATED_SUMMARIES_SQL,
};
use crate::db::schema::{
    BackgroundInfo, BackgroundReference, Bibliography, BibliographyAuthor, Draft, Headline,
    HeadlineChildren, Item, ItemReference, Paragraph, ParagraphSummary, Tag, Task,
};
use crate::{opt_unwrap_or_continue, opt_unwrap_or_return, to_hash_map_key_id};
use sqlx::{Error, SqliteConnection};
use std::collections::HashMap;

impl BackgroundInfo {
    pub async fn load_references(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.references = Some(
            sqlx::query_as::<_, BackgroundReference>(FETCH_BACKGROUND_REFERENCES_SQL)
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
        self.authors = sqlx::query_as::<_, BibliographyAuthor>(FETCH_BIBLIOGRAPHY_AUTHORS_SQL)
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
            sqlx::query_as::<_, ItemReference>(FETCH_ITEM_REFERENCES_SQL)
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_tags(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.tags = Some(
            sqlx::query_as::<_, Tag>(FETCH_ITEM_RELATED_TAGS_SQL)
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
            sqlx::query_as::<_, BackgroundInfo>(FETCH_BACKGROUND_INFO_FROM_ITEM_ID_SQL)
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_tasks(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.tasks = Some(
            sqlx::query_as::<_, Task>(FETCH_ITEM_RELATED_TASKS_SQL)
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
    pub async fn fetch_children(
        &mut self,
        conn: &mut SqliteConnection,
    ) -> Result<Option<HeadlineChildren>, Error> {
        let headlines: HashMap<i64, Headline> = to_hash_map_key_id!(
            sqlx::query_as::<_, Headline>(FETCH_HEADLINE_CHILDREN_RECURSE_SQL)
                .bind(self.id)
                .fetch_all(conn)
                .await?
        );
        let parent = opt_unwrap_or_return!(headlines.get(&self.id).cloned(), Ok(None));
        let mut children: HashMap<i64, Vec<Headline>> = HashMap::new();
        for i in headlines.keys() {
            let headline = opt_unwrap_or_continue!(headlines.get(i));
            match children.get_mut(&opt_unwrap_or_continue!(headline.parent_id)) {
                None => {
                    children.insert(headline.parent_id.unwrap(), vec![headline.clone()]);
                }
                Some(v) => v.push(headline.clone()),
            }
        }
        Ok(Some(HeadlineChildren { parent, children }))
    }

    pub async fn load_paragraph(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.paragraph = Some(
            sqlx::query_as::<_, Paragraph>(FETCH_HEADLINE_RELATED_PARAGRAPH_SQL)
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl Headline {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_paragraph(conn).await
    }
}

impl Paragraph {
    pub async fn load_draft(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.draft = Some(
            sqlx::query_as::<_, Draft>(FETCH_PARAGRAPH_RELATED_DRAFT_SQL)
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_summary(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.summary = Some(
            sqlx::query_as::<_, ParagraphSummary>(FETCH_PARAGRAPH_RELATED_SUMMARIES_SQL)
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
