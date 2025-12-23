use crate::db::schema::{
    BackgroundInfo, BackgroundReference, Bibliography, BibliographyAuthor, Draft, Headline, Item,
    ItemReference, Paragraph, ParagraphSummary, Tag, Task,
};
use sqlx::{Error, Row, SqliteConnection};

pub trait LoadAll {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error>;
}

impl BackgroundInfo {
    pub async fn load_references(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.references = Some(
            sqlx::query_as::<_, BackgroundReference>("")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl LoadAll for BackgroundInfo {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_references(conn).await
    }
}

impl Bibliography {
    pub async fn load_authors(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.authors = sqlx::query_as::<_, BibliographyAuthor>("")
            .bind(self.id)
            .fetch_all(conn)
            .await?;
        Ok(())
    }
}

impl LoadAll for Bibliography {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_authors(conn).await
    }
}

impl Item {
    pub async fn load_references(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.references = Some(
            sqlx::query_as::<_, ItemReference>("SELECT * FROM item_references_list WHERE ?")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_tags(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.tags = Some(
            sqlx::query_as::<_, Tag>("SELECT * FROM item_references_list")
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
            sqlx::query_as::<_, BackgroundInfo>("SELECT * FROM item_references_list")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }

    pub async fn load_tasks(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.tasks = Some(
            sqlx::query_as::<_, Task>("SELECT * FROM item_references_list")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl LoadAll for Item {
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

        sqlx::query_as::<_, Headline>(
            format!(
                "SELECT * FROM headlines WHERE id IN ({})",
                id_list.join(",")
            )
            .as_str(),
        )
        .fetch_all(conn)
        .await
    }

    pub async fn load_paragraph(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.paragraph = Some(
            sqlx::query_as::<_, Paragraph>("SELECT * FROM item_references_list WHERE ?")
                .bind(self.id)
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl LoadAll for Headline {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_children(conn).await?;
        self.load_paragraph(conn).await
    }
}

impl Paragraph {
    pub async fn load_draft(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.draft = Some(sqlx::query_as::<_, Draft>("").fetch_all(conn).await?);
        Ok(())
    }

    pub async fn load_summary(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.summary = Some(
            sqlx::query_as::<_, ParagraphSummary>("")
                .fetch_all(conn)
                .await?,
        );
        Ok(())
    }
}

impl LoadAll for Paragraph {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_draft(conn).await?;
        self.load_summary(conn).await
    }
}
