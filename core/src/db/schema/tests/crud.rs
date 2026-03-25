use crate::db::schema::app_global::book_search_api::BookSearchApi;
use crate::db::schema::app_global::{
    AppGlobalDefaultBibliography, AppGlobalDefaultBibliographyAuthor, AppGlobalDefaultPublisher,
    AppGlobalDefaultRelBibliographyAuthor, AppGlobalDefaultTag, AppGlobalDefaultTaskCategory,
    AppGlobalDefaultTaskTemplate,
};
use crate::db::schema::*;
use crate::test_util::{RandomValue, RandomValueVec};
use sqlx::{SqliteConnection, SqlitePool};
use std::cmp::max;

const TEST_DATA_COUNT: usize = 100;

macro_rules! assert_vec_id_ignored {
    ($left:ident, $right:ident) => {
        let mut l = $left.clone();
        l.iter_mut().for_each(|v| v.id = Default::default());
        let mut r = $right.clone();
        r.iter_mut().for_each(|v| v.id = Default::default());
        assert_eq!(l, r);
    };
}

macro_rules! register_tester {
    ($x:ty, $conn:ident, $records:ident, $result:ident) => {
        assert_vec_id_ignored!($result, $records);
        assert_eq_select_all!($x, $conn, $result);
    };
}

macro_rules! delete_tester {
    ($x:ty, $conn:ident, $result:ident) => {
        let mut current_list = $result.clone();
        while !current_list.is_empty() {
            current_list
                .pop()
                .unwrap()
                .delete(&mut $conn)
                .await
                .unwrap();
            assert_eq_select_all!($x, $conn, current_list);
        }
    };
}

macro_rules! acquire_pool {
    ($pool: ident, $conn: ident) => {
        let mut $conn = $pool.acquire().await.unwrap();
    };
}

macro_rules! assert_eq_select_all {
    ($x:ty, $conn:ident, $val:expr) => {
        let mut v = <$x>::select_all(&mut $conn).await.unwrap();
        assert_eq!(v, $val);
    };
}

macro_rules! test_cr {
    ($x:ty, $pool:ident) => {
        acquire_pool!($pool, conn);
        let mut result = <$x>::register_random_vec(&mut conn, TEST_DATA_COUNT)
            .await
            .unwrap();
        let before = result.before_registration;
        let after = result.after_registration;
        register_tester!($x, conn, before, after);
    };
}

macro_rules! test_cr_prehnite_book {
    ($($x: ty),*) => {
        paste::paste!{$(
        #[sqlx::test(migrator = "crate::db::migrate::prehnite_book::MIGRATOR")]
        async fn [<valid_create_read_random_prehnite_book_ $x:snake>](pool: SqlitePool) {
            test_cr!($x, pool);
        }
        )*}
    };
}

macro_rules! test_cr_app_global {
    ($($x: ty),*) => {
        paste::paste!{$(
        #[sqlx::test(migrator = "crate::db::migrate::app_global::MIGRATOR")]
        async fn [<valid_create_read_random_app_global_ $x:snake>](pool: SqlitePool) {
            test_cr!($x, pool);
        }
        )*}
    };
}

macro_rules! test_crd {
    ($x:ty, $pool:ident) => {
        acquire_pool!($pool, conn);
        let mut result = <$x>::register_random_vec(&mut conn, TEST_DATA_COUNT)
            .await
            .unwrap();
        let before = result.before_registration;
        let after = result.after_registration;
        register_tester!($x, conn, before, after);
        delete_tester!($x, conn, after);
    };
}

macro_rules! test_crd_prehnite_book {
    ($($x: ty),*) => {
        paste::paste!{$(
        #[sqlx::test(migrator = "crate::db::migrate::prehnite_book::MIGRATOR")]
        async fn [<valid_create_read_delete_random_prehnite_book_ $x:snake>](pool: SqlitePool) {
            test_crd!($x, pool);
        })*}
    };
}

macro_rules! test_crd_app_global {
    ($($x: ty),*) => {
        paste::paste!{$(
        #[sqlx::test(migrator = "crate::db::migrate::app_global::MIGRATOR")]
        async fn [<valid_create_read_delete_random_app_global_ $x:snake>](pool: SqlitePool) {
            test_crd!($x, pool);
        })*}
    };
}

#[derive(Clone)]
struct RegisterRandomTestDataResult<T> {
    // 登録前データ
    before_registration: T,
    // 登録後DB出力
    after_registration: T,
}

impl<T> RegisterRandomTestDataResult<T> {
    fn get(self) -> T {
        self.after_registration
    }
}

impl<T> From<(T, T)> for RegisterRandomTestDataResult<T> {
    fn from(value: (T, T)) -> Self {
        RegisterRandomTestDataResult {
            before_registration: value.0,
            after_registration: value.1,
        }
    }
}

macro_rules! return_and_register_random_result {
    ($x:ty, $conn:ident, $records:ident) => {
        return Ok((
            $records.clone(),
            <$x>::register_many($records.as_slice(), $conn, true).await?,
        )
            .into());
    };
}

macro_rules! impl_register_non_dependent_entity {
    ($($x:ty),*) => {
        $(impl $x {
            async fn register_random_vec(conn: &mut SqliteConnection, n: usize) -> Result<RegisterRandomTestDataResult<Vec<$x>>, sqlx::Error> {
                let items = Vec::<$x>::random_n_values(n);
                return_and_register_random_result!($x, conn, items);
            }
        })*
    };
}

macro_rules! random_items {
    ($conn:ident, $n:ident, $items:ident) => {
        let $items = Item::register_random_vec($conn, max(2, $n / 2), ItemType::Paragraph(None))
            .await?
            .get();
        let $items: Vec<Item> = vec![
            Item::register_random_vec($conn, max(1, $n - $items.len()), ItemType::Headline(None))
                .await?
                .get(),
            $items,
        ]
        .into_iter()
        .flatten()
        .collect();
    };
}

impl_register_non_dependent_entity!(
    BackgroundInfo,
    Tag,
    Publisher,
    BibliographyAuthor,
    TaskCategory,
    Setting
);

impl_register_non_dependent_entity!(BookSearchApi);

impl BackgroundReference {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<BackgroundReference>>, sqlx::Error> {
        let background_info = BackgroundInfo::register_random_vec(conn, max(3, n))
            .await?
            .get();
        let bibliography = Bibliography::register_random_vec(conn, max(3, n))
            .await?
            .get();
        let mut references = Vec::<BackgroundReference>::random_n_values(max(3usize, n));
        references.iter_mut().enumerate().for_each(|(i, v)| {
            v.bibliography = bibliography[i].clone();
            v.background_info_id = background_info[i].id;
        });
        references.push(BackgroundReference {
            background_info_id: background_info[0].id,
            bibliography: bibliography[1].clone(),
            ..BackgroundReference::random_value()
        });
        return_and_register_random_result!(BackgroundReference, conn, references);
    }
}

impl Bibliography {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<Bibliography>>, sqlx::Error> {
        let publishers = Publisher::register_many(
            Vec::<Publisher>::random_n_values(n / 4).as_slice(),
            conn,
            true,
        )
        .await?;
        let mut bibliographies = Vec::<Bibliography>::random_n_values(n);
        bibliographies.chunks_mut(2).enumerate().for_each(|(i, v)| {
            v.iter_mut().enumerate().for_each(|(j, v)| {
                v.publisher = publishers.get(i).cloned();
            });
        });
        return_and_register_random_result!(Bibliography, conn, bibliographies);
    }
}

impl Draft {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<Draft>>, sqlx::Error> {
        let paragraph = Paragraph::register_random_vec(conn, n / 4).await?.get();
        let mut draft = Vec::<Draft>::random_n_values(n);
        draft.chunks_mut(4).enumerate().for_each(|(i, v)| {
            v.iter_mut().for_each(|v| {
                v.paragraph_id = paragraph
                    .get(i)
                    .unwrap_or_else(|| paragraph.first().unwrap())
                    .id
            })
        });
        return_and_register_random_result!(Draft, conn, draft);
    }
}

impl Headline {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<Headline>>, sqlx::Error> {
        let items = Item::register_random_vec(conn, n, ItemType::Headline(None))
            .await?
            .get();
        let v = Headline::register_random_vec_with_item(conn, n, &items).await?;
        let (root_before, mut before) = v.before_registration;
        let (root_after, mut after) = v.after_registration;
        before.insert(0, root_before);
        after.insert(0, root_after);
        Ok((before, after).into())
    }

    async fn register_random_vec_with_item(
        conn: &mut SqliteConnection,
        n: usize,
        p_items: &Vec<Item>,
    ) -> Result<RegisterRandomTestDataResult<(Headline, Vec<Headline>)>, sqlx::Error> {
        let items: Vec<Item> = p_items
            .iter()
            .filter_map(|v| match v.item_type {
                ItemType::Headline(_) => Some(v.clone()),
                ItemType::Paragraph(_) => None,
            })
            .collect();
        let before_reg_root = Headline {
            item_id: items[0].id,
            ..Headline::random_value()
        };
        let root = before_reg_root.register(conn, true).await?;

        let mut headlines = Vec::<Headline>::random_n_values(n - 1);
        if n > 1 {
            headlines.iter_mut().enumerate().for_each(|(i, v)| {
                v.item_id = items[i + 1].id;
                v.parent_id = Some(root.id);
            });
        }
        Ok((
            (before_reg_root, headlines.clone()),
            (
                root,
                Headline::register_many(headlines.as_slice(), conn, true).await?,
            ),
        )
            .into())
    }
}

impl Paragraph {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<Paragraph>>, sqlx::Error> {
        let items = Item::register_random_vec(conn, n, ItemType::Paragraph(None))
            .await?
            .get();
        let headline_items = Item::register_random_vec(conn, n / 4, ItemType::Headline(None))
            .await?
            .get();
        let headlines =
            Headline::register_random_vec_with_item(conn, n / 4, &headline_items).await?;
        Paragraph::register_random_vec_with_item(conn, n, &items, &headlines).await
    }

    async fn register_random_vec_with_item(
        conn: &mut SqliteConnection,
        n: usize,
        p_items: &Vec<Item>,
        p_headlines: &RegisterRandomTestDataResult<(Headline, Vec<Headline>)>,
    ) -> Result<RegisterRandomTestDataResult<Vec<Paragraph>>, sqlx::Error> {
        let items: Vec<Item> = p_items
            .iter()
            .filter_map(|v| match v.item_type {
                ItemType::Headline(_) => None,
                ItemType::Paragraph(_) => Some(v.clone()),
            })
            .collect();
        let (root_headline, headlines) = p_headlines.clone().get();
        let mut paragraph = Vec::<Paragraph>::random_n_values(n);
        paragraph.chunks_mut(4).enumerate().for_each(|(i, v)| {
            v.iter_mut().enumerate().for_each(|(j, v)| {
                v.item_id = items[i * 4 + j].id;
                if i == 0 {
                    v.headline = root_headline.clone();
                } else {
                    v.headline = headlines.get(i - 1).unwrap_or(&root_headline).clone();
                }
            })
        });
        return_and_register_random_result!(Paragraph, conn, paragraph);
    }
}

impl Item {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
        item_type: ItemType,
    ) -> Result<RegisterRandomTestDataResult<Vec<Item>>, sqlx::Error> {
        let mut items: Vec<Item> = Vec::<Item>::random_n_values(n)
            .into_iter()
            .map(|mut v| {
                v.item_type = match item_type {
                    ItemType::Headline(_) => ItemType::Headline(None),
                    ItemType::Paragraph(_) => ItemType::Paragraph(None),
                };
                v
            })
            .collect();
        return_and_register_random_result!(Item, conn, items);
    }
}

impl ItemReference {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<ItemReference>>, sqlx::Error> {
        random_items!(conn, n, items);
        let bibliography = Bibliography::register_random_vec(conn, max(3, n))
            .await?
            .get();
        let mut references = Vec::<ItemReference>::random_n_values(max(3usize, n));
        references.iter_mut().enumerate().for_each(|(i, v)| {
            v.bibliography = bibliography[i].clone();
            v.item_id = items[i].id;
        });
        references.push(ItemReference {
            item_id: items[0].id,
            bibliography: bibliography[1].clone(),
            ..ItemReference::random_value()
        });
        return_and_register_random_result!(ItemReference, conn, references);
    }
}

impl ParagraphLink {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<ParagraphLink>>, sqlx::Error> {
        let paragraph_a = Paragraph::register_random_vec(conn, max(3, n - 2))
            .await?
            .get();
        let paragraph_b = Paragraph::register_random_vec(conn, max(3, n - 2))
            .await?
            .get();
        let tasks = Task::register_random_vec(conn, n / 2).await?.get();
        let mut references = vec![ParagraphLink::default(); max(3usize, n - 2)];
        references.iter_mut().enumerate().for_each(|(i, v)| {
            v.to_paragraph = paragraph_a[i].clone();
            v.from_paragraph = paragraph_b[i].clone();
            v.task = tasks.get(i).cloned();
        });
        references.push(ParagraphLink {
            to_paragraph: paragraph_a[0].clone(),
            from_paragraph: paragraph_b[1].clone(),
            ..Default::default()
        });
        references.push(ParagraphLink {
            to_paragraph: paragraph_b[0].clone(),
            from_paragraph: paragraph_a[1].clone(),
            ..Default::default()
        });
        return_and_register_random_result!(ParagraphLink, conn, references);
    }
}

impl ParagraphSummary {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<ParagraphSummary>>, sqlx::Error> {
        let paragraph = Paragraph::register_random_vec(conn, n / 4).await?.get();
        let mut summaries = Vec::<ParagraphSummary>::random_n_values(n);
        summaries.chunks_mut(4).enumerate().for_each(|(i, v)| {
            v.iter_mut().for_each(|v| {
                v.paragraph_id = paragraph
                    .get(i)
                    .unwrap_or_else(|| paragraph.first().unwrap())
                    .id
            })
        });
        return_and_register_random_result!(ParagraphSummary, conn, summaries);
    }
}

impl RelBackgroundAndItem {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<RelBackgroundAndItem>>, sqlx::Error> {
        let background_info = BackgroundInfo::register_random_vec(conn, max(3, n))
            .await?
            .get();
        random_items!(conn, n, items);
        let mut references = vec![RelBackgroundAndItem::default(); max(3usize, n)];
        references.iter_mut().enumerate().for_each(|(i, v)| {
            v.background_info_id = background_info[i].id;
            v.item_id = items[i].id;
        });
        references.push(RelBackgroundAndItem {
            background_info_id: background_info[0].id,
            item_id: items[1].id,
            ..Default::default()
        });
        return_and_register_random_result!(RelBackgroundAndItem, conn, references);
    }
}

impl RelBibliographyAuthor {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<RelBibliographyAuthor>>, sqlx::Error> {
        let authors = BibliographyAuthor::register_random_vec(conn, max(3, n))
            .await?
            .get();
        let bibliography = Bibliography::register_random_vec(conn, max(3, n))
            .await?
            .get();
        let mut references = vec![RelBibliographyAuthor::default(); max(3usize, n)];
        references.iter_mut().enumerate().for_each(|(i, v)| {
            v.bibliography_author_id = authors[i].id;
            v.bibliography_id = bibliography[i].id;
        });
        references.push(RelBibliographyAuthor {
            bibliography_id: authors[0].id,
            bibliography_author_id: bibliography[1].id,
            ..Default::default()
        });
        return_and_register_random_result!(RelBibliographyAuthor, conn, references);
    }
}

impl RelTagAndItem {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<RelTagAndItem>>, sqlx::Error> {
        let tags = Tag::register_random_vec(conn, max(3, n)).await?.get();
        random_items!(conn, n, items);
        let mut references = vec![RelTagAndItem::default(); max(3usize, n)];
        references.iter_mut().enumerate().for_each(|(i, v)| {
            v.tag_id = tags[i].id;
            v.item_id = items[i].id;
        });
        references.push(RelTagAndItem {
            tag_id: tags[0].id,
            item_id: items[1].id,
            ..Default::default()
        });
        return_and_register_random_result!(RelTagAndItem, conn, references);
    }
}

impl Task {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<Task>>, sqlx::Error> {
        let categories = TaskCategory::register_random_vec(conn, n / 2).await?.get();
        let items: Vec<Item> = vec![
            Item::register_random_vec(conn, n / 8, ItemType::Paragraph(None))
                .await?
                .get(),
            Item::register_random_vec(conn, n / 8, ItemType::Headline(None))
                .await?
                .get(),
        ]
        .into_iter()
        .flatten()
        .collect();
        let mut tasks = Vec::<Task>::random_n_values(n);
        tasks.chunks_mut(4).enumerate().for_each(|(i, v)| {
            v.iter_mut().enumerate().for_each(|(j, v)| {
                v.item_id = items.get(i).unwrap_or_else(|| items.first().unwrap()).id;
                v.task_category = categories.get(i * 4 + j).cloned();
            });
        });
        return_and_register_random_result!(Task, conn, tasks);
    }
}

impl TaskTemplate {
    async fn register_random_vec(
        conn: &mut SqliteConnection,
        n: usize,
    ) -> Result<RegisterRandomTestDataResult<Vec<TaskTemplate>>, sqlx::Error> {
        let categories = TaskCategory::register_random_vec(conn, n / 2).await?.get();
        let mut templates = Vec::<TaskTemplate>::random_n_values(n);
        templates
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| v.task_category = categories.get(i).cloned());
        return_and_register_random_result!(TaskTemplate, conn, templates);
    }
}

test_crd_prehnite_book!(
    BackgroundInfo,
    Tag,
    Publisher,
    BibliographyAuthor,
    TaskCategory,
    Setting
);

test_crd_app_global!(
    AppGlobalDefaultPublisher,
    AppGlobalDefaultTaskCategory,
    AppGlobalDefaultTag,
    AppGlobalDefaultBibliographyAuthor,
    BookSearchApi
);

test_cr_prehnite_book!(
    BackgroundReference,
    Bibliography,
    Draft,
    Headline,
    ItemReference,
    Paragraph,
    ParagraphLink,
    ParagraphSummary,
    RelBackgroundAndItem,
    RelBibliographyAuthor,
    RelTagAndItem,
    Task,
    TaskTemplate
);

test_cr_app_global!(
    AppGlobalDefaultBibliography,
    AppGlobalDefaultTaskTemplate,
    AppGlobalDefaultRelBibliographyAuthor
);

#[sqlx::test(migrator = "crate::db::migrate::prehnite_book::MIGRATOR")]
async fn valid_create_read_random_item(pool: SqlitePool) {
    const DATA_COUNT: usize = TEST_DATA_COUNT / 2;
    acquire_pool!(pool, conn);
    // 親を登録
    let paragraph_items =
        Item::register_random_vec(&mut conn, DATA_COUNT, ItemType::Paragraph(None))
            .await
            .unwrap();
    let headline_items = Item::register_random_vec(&mut conn, DATA_COUNT, ItemType::Headline(None))
        .await
        .unwrap();
    let before_reg_items = vec![
        paragraph_items.before_registration.clone(),
        headline_items.before_registration.clone(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<Item>>();
    let mut items = vec![paragraph_items.get(), headline_items.get()]
        .into_iter()
        .flatten()
        .collect::<Vec<Item>>();
    register_tester!(Item, conn, before_reg_items, items);
    let headlines = Headline::register_random_vec_with_item(&mut conn, DATA_COUNT, &items)
        .await
        .unwrap();
    let (h_root, mut headlines_vec) = headlines.clone().get();
    headlines_vec.insert(0, h_root);
    let paragraph =
        Paragraph::register_random_vec_with_item(&mut conn, DATA_COUNT, &items, &headlines)
            .await
            .unwrap()
            .get();
    items
        .iter_mut()
        .filter(|v| match v.item_type {
            ItemType::Headline(_) => true,
            ItemType::Paragraph(_) => false,
        })
        .enumerate()
        .for_each(|(i, v)| v.item_type = ItemType::Headline(headlines_vec.get(i).cloned()));
    items
        .iter_mut()
        .filter(|v| match v.item_type {
            ItemType::Headline(_) => false,
            ItemType::Paragraph(_) => true,
        })
        .enumerate()
        .for_each(|(i, v)| v.item_type = ItemType::Paragraph({ Some(paragraph[i].clone()) }));
    assert_eq_select_all!(Item, conn, items);
}
