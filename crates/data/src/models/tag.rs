use crate::database::schema::tag::DbTag;
use crate::manager::managed::{PrehniteManaged, PrehniteManagedItem};
use crate::util::{FgBg, find_random_fb_color_pair};
use crate::{TString, TagId};
use palette::Hsla;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TagMetadata {
    pub foreground_color: Hsla,
    pub background_color: Hsla,
}

impl TagMetadata {
    pub fn set_random_color(&mut self) {
        let Self {
            foreground_color,
            background_color,
            ..
        } = Self::default();
        self.foreground_color = foreground_color;
        self.background_color = background_color;
    }
}

impl Default for TagMetadata {
    fn default() -> Self {
        let FgBg {
            foreground_color,
            background_color,
        } = find_random_fb_color_pair::<Hsla>();
        TagMetadata {
            foreground_color,
            background_color,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    pub id: TagId,
    pub title: TString,
    pub details: Option<TString>,
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
        let title = title.into();
        let details = details.map(Into::into);
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
        let title = title.into();
        let details = details.map(Into::into);
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
    async fn save(&mut self) -> crate::Result<()> {
        self.clone()
            .to_db()
            .save(&mut *self.manager().acquire().await?)
            .await?;
        Ok(())
    }

    pub async fn delete(&mut self) -> crate::Result<bool> {
        self.clone().to_db().delete(&mut *self.manager().acquire().await?).await
    }

    pub async fn update_title(&mut self, title: &str) -> crate::Result<()> {
        self.as_mut().title = title.into();
        self.save().await
    }

    pub async fn update_details(&mut self, details: Option<&str>) -> crate::Result<()> {
        self.as_mut().details = details.map(|x| x.into());
        self.save().await
    }

    pub async fn update_metadata(&mut self, metadata: TagMetadata) -> crate::Result<()> {
        self.as_mut().metadata = metadata;
        self.save().await
    }
}

impl PrehniteManagedItem for Tag {}
