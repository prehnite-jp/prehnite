use crate::manager::weak_inner_data_manager::WeakInnerDataManager;

#[cfg(feature = "gpui")]
pub use gpui_impl::TagStore;

/// タグを管理するリポジトリ
#[derive(Clone)]
pub struct TagRepository {
    manager: WeakInnerDataManager,
}

impl TagRepository {
    pub(crate) fn new(manager: WeakInnerDataManager) -> Self {
        Self { manager }
    }
}

#[cfg(not(feature = "gpui"))]
mod _impl {
    use crate::database::schema::tag::DbTag;
    use crate::manager::PrehniteManaged;
    use crate::{TString, Tag, TagId, TagMetadata, TagRepository};
    use serde::{Deserialize, Serialize};
    use sqlx::Acquire;

    impl TagRepository {
        /// タグを新しく作成します。titleが同じタグが存在する場合は代わりに取得します。
        pub async fn fetch_or_create(
            &self,
            title: &str,
            details: Option<&str>,
            metadata: Option<TagMetadata>,
        ) -> crate::Result<PrehniteManaged<Tag>> {
            let mut conn = self.manager.acquire().await?;
            let mut tx = conn.begin().await?;
            let res = DbTag::fetch_or_create(title, details, metadata, &mut *tx).await?;
            tx.commit().await?;
            Ok(self.manager.manage(Tag::from(res)))
        }

        /// 全てのタグを取得します。
        pub async fn list(&self) -> crate::Result<Vec<Tag>> {
            let mut conn = self.manager.acquire().await?;
            let res = DbTag::fetch_all(&mut *conn).await?;
            Ok(res.into_iter().map(Tag::from).collect())
        }

        /// idに一致するタグを取得します。 存在しない場合、Err(Error::MissingTag)が返されます。
        pub async fn fetch_by_id(&self, tag_id: TagId) -> crate::Result<PrehniteManaged<Tag>> {
            let mut conn = self.manager.acquire().await?;
            Ok(self.manager.manage(
                DbTag::fetch_by_id(tag_id, &mut *conn)
                    .await?
                    .ok_or(crate::Error::MissingTag(tag_id.to_string()))?
                    .into(),
            ))
        }

        /// titleに一致するタグを取得します。 存在しない場合、Err(Error::MissingTag)が返されます。
        pub async fn fetch_by_title(&self, title: &str) -> crate::Result<PrehniteManaged<Tag>> {
            let mut conn = self.manager.acquire().await?;
            Ok(self.manager.manage(
                DbTag::fetch(&title, &mut *conn)
                    .await?
                    .ok_or(crate::Error::MissingTag(title.into()))?
                    .into(),
            ))
        }

        /// エクスポートされたJSON形式のタグを取り込みます。(IDは無視されます。)
        ///
        /// TODO: IDが存在する場合の処理を実装する。
        pub async fn import_from_json(
            &self,
            json: &str,
        ) -> crate::Result<Vec<PrehniteManaged<Tag>>> {
            #[derive(Deserialize, Serialize)]
            #[serde(untagged)]
            enum ImportableTag {
                Array(Vec<ImportableTagInner>),
                Tag(ImportableTagInner),
            }

            impl ImportableTag {
                pub fn into_vec(self) -> Vec<ImportableTagInner> {
                    match self {
                        ImportableTag::Array(x) => x,
                        ImportableTag::Tag(x) => vec![x],
                    }
                }
            }

            #[derive(Deserialize, Serialize)]
            struct ImportableTagInner {
                id: Option<TagId>,
                title: TString,
                details: Option<TString>,
                metadata: Option<TagMetadata>,
            }

            let importable: ImportableTag = serde_json::from_str(json)?;
            let importable: Vec<ImportableTagInner> = importable.into_vec();
            let mut res = vec![];
            for i in importable.into_iter() {
                res.push(
                    self.fetch_or_create(
                        i.title.as_ref(),
                        i.details.as_ref().map(TString::as_str),
                        i.metadata,
                    )
                    .await?,
                );
            }
            Ok(res)
        }

        /// 全てのタグをJSONとしてエクスポートします。
        pub async fn export_to_json(&self) -> crate::Result<String> {
            Ok(serde_json::to_string(&self.list().await?)?)
        }
    }
}

#[cfg(feature = "gpui")]
mod gpui_impl {
    use crate::database::schema::tag::DbTag;
    use crate::manager::{DataManagerStore, PrehniteManaged};
    use crate::{TString, Tag, TagId, TagMetadata, TagRepository};
    use gpui_kit::*;
    use serde::{Deserialize, Serialize};
    use sqlx::Acquire;
    use std::collections::BTreeMap;
    use std::ops::Deref;
    use tracing_unwrap::ResultExt;

    #[derive(Clone, Debug, Default)]
    pub struct TagStore(BTreeMap<TagId, Tag>);

    impl Deref for TagStore {
        type Target = BTreeMap<TagId, Tag>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl TagStore {
        pub(crate) fn init(cx: &mut App) {
            cx.set_global(Self::default());
        }

        pub fn reload(cx: &mut App) -> crate::Result<()> {
            let data_manager = DataManagerStore::get_owned(cx)?;
            cx.spawn(async move |cx| {
                let tags = data_manager
                    .tags()
                    .list(cx)
                    .await
                    .ok_or_log()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| (x.id, x))
                    .collect();
                cx.update_global::<TagStore, _>(|x, _| {
                    x.0 = tags;
                });
            })
            .detach();
            Ok(())
        }

        pub(crate) fn clear(cx: &mut App) {
            cx.set_global(Self::default());
        }

        pub fn global(cx: &mut App) -> &TagStore {
            cx.global()
        }
    }

    impl Global for TagStore {}

    impl TagRepository {
        /// タグを新しく作成します。titleが同じタグが存在する場合は代わりに取得します。
        pub fn fetch_or_create<C: AppContext>(
            &self,
            cx: &mut C,
            title: TString,
            details: Option<TString>,
            metadata: Option<TagMetadata>,
        ) -> Task<crate::Result<PrehniteManaged<Tag>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let mut conn = repos.manager.acquire().await?;
                let mut tx = conn.begin().await?;
                let res = DbTag::fetch_or_create(
                    &title,
                    details.as_ref().map(AsRef::as_ref),
                    metadata,
                    &mut *tx,
                )
                .await?;
                tx.commit().await?;
                Ok(repos.manager.manage(Tag::from(res)))
            })
        }

        /// 全てのタグを取得します。
        pub fn list<C: AppContext>(&self, cx: &mut C) -> Task<crate::Result<Vec<Tag>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let mut conn = repos.manager.acquire().await?;
                let res = DbTag::fetch_all(&mut *conn).await?;
                Ok(res.into_iter().map(Tag::from).collect())
            })
        }

        /// idに一致するタグを取得します。 存在しない場合、Err(Error::MissingTag)が返されます。
        pub fn fetch_by_id<C: AppContext>(
            &self,
            cx: &mut C,
            tag_id: TagId,
        ) -> Task<crate::Result<PrehniteManaged<Tag>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let mut conn = repos.manager.acquire().await?;
                Ok(repos.manager.manage(
                    DbTag::fetch_by_id(tag_id, &mut *conn)
                        .await?
                        .ok_or(crate::Error::MissingTag(tag_id.to_string()))?
                        .into(),
                ))
            })
        }

        /// titleに一致するタグを取得します。 存在しない場合、Err(Error::MissingTag)が返されます。
        pub fn fetch_by_title<C: AppContext>(
            &self,
            cx: &mut C,
            title: String,
        ) -> Task<crate::Result<PrehniteManaged<Tag>>> {
            let repos = self.clone();
            cx.background_spawn(async move {
                let mut conn = repos.manager.acquire().await?;
                Ok(repos.manager.manage(
                    DbTag::fetch(&title, &mut *conn)
                        .await?
                        .ok_or(crate::Error::MissingTag(title.into()))?
                        .into(),
                ))
            })
        }

        /// エクスポートされたJSON形式のタグを取り込みます。(IDは無視されます。)
        ///
        /// TODO: IDが存在する場合の処理を実装する。
        pub fn import_from_json(
            &self,
            cx: &mut App,
            json: String,
        ) -> Task<crate::Result<Vec<PrehniteManaged<Tag>>>> {
            #[derive(Deserialize, Serialize)]
            #[serde(untagged)]
            enum ImportableTag {
                Array(Vec<ImportableTagInner>),
                Tag(ImportableTagInner),
            }

            impl ImportableTag {
                pub fn into_vec(self) -> Vec<ImportableTagInner> {
                    match self {
                        ImportableTag::Array(x) => x,
                        ImportableTag::Tag(x) => vec![x],
                    }
                }
            }

            #[derive(Deserialize, Serialize)]
            struct ImportableTagInner {
                id: Option<TagId>,
                title: TString,
                details: Option<TString>,
                metadata: Option<TagMetadata>,
            }
            let repos = self.clone();
            cx.spawn(async move |cx| {
                let importable: ImportableTag = serde_json::from_str(&json)?;
                let importable: Vec<ImportableTagInner> = importable.into_vec();
                let mut res = vec![];
                for i in importable.into_iter() {
                    res.push(
                        repos
                            .fetch_or_create(
                                cx,
                                i.title,
                                i.details,
                                i.metadata,
                            )
                            .await?,
                    );
                }
                Ok(res)
            })
        }

        /// 全てのタグをJSONとしてエクスポートします。
        pub fn export_to_json(&self, cx: &mut App) -> Task<crate::Result<String>> {
            let repos = self.clone();
            cx.spawn(async move |cx| Ok(serde_json::to_string(&repos.list(cx).await?)?))
        }
    }
}
