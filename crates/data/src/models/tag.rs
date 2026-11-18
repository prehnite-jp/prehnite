use crate::TagId;
use crate::database::schema::tag::DbTag;
use crate::manager::managed::{PrehniteManaged, PrehniteManagedItem};
use random_color::options::Luminosity;
use rgb::RGB8;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TagMetadata {
    pub foreground_color: RGB8,
    pub background_color: RGB8,
}

impl TagMetadata {
    pub fn set_random_color(self) -> Self {
        let Self {
            foreground_color,
            background_color,
            ..
        } = Self::default();
        Self {
            foreground_color,
            background_color,
            ..self
        }
    }
}

impl Default for TagMetadata {
    fn default() -> Self {
        TagMetadata {
            foreground_color: random_color::RandomColor::new()
                .luminosity(Luminosity::Light)
                .to_rgb(),
            background_color: random_color::RandomColor::new()
                .luminosity(Luminosity::Dark)
                .to_rgb(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Tag {
    pub id: TagId,
    pub title: String,
    pub details: Option<String>,
    pub metadata: TagMetadata,
}

impl From<DbTag> for Tag {
    fn from(value: DbTag) -> Self {
        let DbTag {
            id,
            title,
            details,
            metadata: Json(metadata),
        } = value;
        Self {
            id,
            title,
            details,
            metadata,
        }
    }
}

impl From<Tag> for DbTag {
    fn from(value: Tag) -> Self {
        let Tag {
            id,
            title,
            details,
            metadata,
        } = value;
        let metadata = Json(metadata);
        DbTag {
            id,
            title,
            details,
            metadata,
        }
    }
}

impl Tag {
    fn to_db(self) -> DbTag {
        self.into()
    }
}

impl PrehniteManaged<Tag> {
    async fn save(self) -> crate::Result<Self> {
        self.clone()
            .to_db()
            .save(&mut *self.manager().acquire().await?)
            .await?;
        Ok(self)
    }

    pub async fn delete(self) -> crate::Result<bool> {
        let mut conn = self.manager().acquire().await?;
        self.item().to_db().delete(&mut *conn).await
    }

    pub async fn update_title(mut self, title: String) -> crate::Result<Self> {
        self.as_mut().title = title;
        let new = Tag { ..self.clone() };
        self.save().await
    }

    pub async fn update_details(mut self, details: Option<String>) -> crate::Result<Self> {
        self.as_mut().details = details;
        self.save().await
    }

    pub async fn update_metadata(mut self, metadata: TagMetadata) -> crate::Result<Self> {
        self.as_mut().metadata = metadata;
        self.save().await
    }
}

impl PrehniteManagedItem for Tag {}
