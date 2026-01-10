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

const FETCH_CHILDREN_CTE_QUERY: &str = r#"
WITH RECURSIVE children(p_id) AS (
 VALUES (?) UNION ALL SELECT headlines.id FROM headlines
 LEFT OUTER JOIN children ON headlines.parent_id = children.p_id
 WHERE headlines.parent_id = p_id
) SELECT * FROM headlines WHERE id IN (SELECT * FROM children);"#;

impl Headline {
    pub async fn fetch_children(
        &mut self,
        conn: &mut SqliteConnection,
    ) -> Result<Option<HeadlineChildren>, Error> {
        let headlines: HashMap<i64, Headline> = to_hash_map_key_id!(
            sqlx::query_as::<_, Headline>(FETCH_CHILDREN_CTE_QUERY)
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
