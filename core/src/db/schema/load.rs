use crate::db::query;
use crate::db::schema::{
    BackgroundInfo, Bibliography, Headline, Item, Paragraph,
};
use sqlx::{Error, SqliteConnection};

impl BackgroundInfo {
    pub async fn load_references(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.references = Some(query::fetch_background_references(conn, self.id).await?);
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
        self.authors = query::fetch_bibliography_authors(conn, self.id).await?;
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
        self.references = Some(query::fetch_item_references(conn, self.id).await?);
        Ok(())
    }

    pub async fn load_tags(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.tags = Some(query::fetch_item_related_tags(conn, self.id).await?);
        Ok(())
    }

    pub async fn load_background_info_list(
        &mut self,
        conn: &mut SqliteConnection,
    ) -> Result<(), Error> {
        self.background_info_list =
            Some(query::fetch_background_info_from_item_id(conn, self.id).await?);
        Ok(())
    }

    pub async fn load_tasks(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.tasks = Some(query::fetch_item_related_tasks(conn, self.id).await?);
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
    pub async fn load_paragraph(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.paragraph = Some(query::fetch_headline_related_paragraph(conn, self.id).await?);
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
        self.draft = Some(query::fetch_paragraph_related_draft(conn, self.id).await?);
        Ok(())
    }

    pub async fn load_summary(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.summary = Some(query::fetch_paragraph_related_summaries(conn, self.id).await?);
        Ok(())
    }
}

impl Paragraph {
    async fn load_all(&mut self, conn: &mut SqliteConnection) -> Result<(), Error> {
        self.load_draft(conn).await?;
        self.load_summary(conn).await
    }
}
