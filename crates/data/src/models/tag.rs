use crate::database::schema::tag::DbTag;
use crate::manager::managed::{PrehniteManaged, PrehniteManagedItem};
use crate::util::{FgBg, find_random_fb_color_pair};
use crate::{TString, TagId};
use palette::Hsla;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[cfg(feature = "gpui")]
pub use _gpui_impl::TagList;

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

#[cfg_attr(feature = "gpui", derive(gpui_kit::IntoElement))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    pub id: TagId,
    pub title: TString,
    pub details: Option<TString>,
    pub metadata: TagMetadata,
}

impl Tag {
    fn to_db(self) -> DbTag {
        self.into()
    }
}

impl PrehniteManaged<Tag> {
    async fn save(&mut self) -> crate::Result<()> {
        self.clone_item()
            .to_db()
            .save(&mut *self.manager().acquire().await?)
            .await?;
        Ok(())
    }

    pub async fn delete(&mut self) -> crate::Result<bool> {
        self.clone_item()
            .to_db()
            .delete(&mut *self.manager().acquire().await?)
            .await
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

#[cfg(feature = "gpui")]
mod _gpui_impl {
    use crate::manager::PrehniteManagedEntity;
    use crate::{Tag, TagId, TagMetadata, TagStore};
    use gpui_kit::component::tooltip::Tooltip;
    use gpui_kit::*;
    use prehnite_shared::util::ToGpui;

    impl RenderOnce for Tag {
        fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
            let theme = component::Theme::global(cx);
            let result = div().id(("tag", *self.id as u64)).child(
                component::tag::Tag::custom(
                    self.metadata.foreground_color.to_gpui(),
                    self.metadata.background_color.to_gpui(),
                    theme.border,
                )
                .border_1()
                .text_center()
                .child(self.title),
            );
            if let Some(details) = self.details {
                result.tooltip(move |window, cx| Tooltip::new(details.clone()).build(window, cx))
            } else {
                result
            }
        }
    }

    #[derive(Debug, Clone, IntoElement)]
    pub struct TagList(Vec<TagId>);

    impl TagList {
        pub fn new(list: Vec<TagId>) -> TagList {
            Self(list)
        }
    }

    impl RenderOnce for TagList {
        fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
            let tags = TagStore::global(cx);
            div()
                .id("tag-list")
                .flex()
                .flex_row()
                .gap(px(2.))
                .w_full()
                .overflow_x_scroll()
                .children(self.0.iter().filter_map(|x| tags.get(x).cloned()))
        }
    }

    impl PrehniteManagedEntity<Tag> {
        pub fn delete(&self, cx: &mut App) -> Task<crate::Result<bool>> {
            let entity = self.clone();
            cx.spawn(async move |cx| {
                let managed = entity.clone_inner(cx);
                let item = managed.clone_item().to_db();
                let mut conn = managed.manager().acquire().await?;
                let res = cx
                    .background_spawn(async move { item.delete(&mut *conn).await })
                    .await?;
                entity.set_deleted(cx, true);
                Ok(res)
            })
        }

        pub fn update_title(&self, cx: &mut App, title: &str) -> Task<crate::Result<()>> {
            let entity = self.clone();
            let title = title.into();
            cx.spawn(async move |cx| {
                let mut managed = entity.clone_inner(cx);
                let item = managed.clone_item().to_db();
                managed.as_mut().title = title;
                let mut conn = managed.manager().acquire().await?;
                cx.background_spawn(async move { item.save(&mut *conn).await })
                    .await?;
                entity.update(cx, move |v, cx| {
                    *v = managed;
                    cx.notify();
                });
                Ok(())
            })
        }

        pub fn update_details(
            &self,
            cx: &mut App,
            details: Option<&str>,
        ) -> Task<crate::Result<()>> {
            let entity = self.clone();
            let details = details.map(Into::into);
            cx.spawn(async move |cx| {
                let mut managed = entity.clone_inner(cx);
                let item = managed.clone_item().to_db();
                managed.as_mut().details = details;
                let mut conn = managed.manager().acquire().await?;
                cx.background_spawn(async move { item.save(&mut *conn).await })
                    .await?;
                entity.update(cx, move |v, cx| {
                    *v = managed;
                    cx.notify();
                });
                Ok(())
            })
        }

        pub fn update_metadata(
            &self,
            cx: &mut App,
            metadata: TagMetadata,
        ) -> Task<crate::Result<()>> {
            let entity = self.clone();
            cx.spawn(async move |cx| {
                let mut managed = entity.clone_inner(cx);
                let item = managed.clone_item().to_db();
                managed.as_mut().metadata = metadata;
                let mut conn = managed.manager().acquire().await?;
                cx.background_spawn(async move { item.save(&mut *conn).await })
                    .await?;
                entity.update(cx, move |v, cx| {
                    *v = managed;
                    cx.notify();
                });
                Ok(())
            })
        }
    }
}
