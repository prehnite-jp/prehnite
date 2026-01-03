use crate::db::util::cushion_types::{OptionString, VecString};
use crate::db::schema::{Bibliography, BibliographyAuthor, Publisher, RelBibliographyAuthor};
use crate::db::util::get_optional;
use rhai::{CustomType, Dynamic, EvalAltResult, Position, TypeBuilder};
use sqlx::{Acquire, SqliteConnection};
use std::collections::HashMap;

#[derive(Default, Clone, CustomType, Debug)]
#[derive(PartialEq)]
pub struct BookSearchResult {
    pub isbn: Option<String>,
    pub url: Option<String>,
    pub title: String,
    pub detail: Option<String>,
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub publication_date: Option<String>,
}

impl BookSearchResult {
    pub(crate) fn new(
        isbn: Dynamic,
        url: Dynamic,
        title: String,
        detail: Dynamic,
        authors: Dynamic,
        publisher: Dynamic,
        publication_date: Dynamic,
    ) -> Self {
        BookSearchResult {
            isbn: OptionString::from(isbn).into(),
            url: OptionString::from(url).into(),
            title,
            detail: OptionString::from(detail).into(),
            authors: VecString::from(authors).into(),
            publisher: OptionString::from(publisher).into(),
            publication_date: OptionString::from(publication_date).into(),
        }
    }

    pub(crate) fn example() -> Self {
        BookSearchResult {
            isbn: Some("000-0-00-000000-0".to_string()),
            url: None,
            title: "The Book of Example".to_string(),
            detail: Some(
                "This book is an example! This Book Search API configuration is for illustrative purposes only and cannot be used.".to_string(),
            ),
            authors: vec![
                "Hanako T".to_string(),
                "Taro T".to_string()
            ],
            publisher: Some("FooBar Printing Exm".to_string()),
            publication_date: Some("0000-01-01".to_string()),
        }
    }

    pub async fn register(
        conn: &mut SqliteConnection,
        book_search_result_list: Vec<BookSearchResult>,
    ) -> Result<Vec<Bibliography>, sqlx::Error> {
        let mut tx = conn.begin().await?;
        let publishers: HashMap<String, Publisher> = Publisher::register_vec_tx(
            book_search_result_list
                .iter()
                .filter_map(|v| {
                    Some(Publisher {
                        id: 0,
                        name: v.publisher.clone()?,
                        memo: None,
                    })
                })
                .collect::<Vec<Publisher>>()
                .as_slice(),
            &mut tx,
            true,
        )
            .await?
            .into_iter()
            .map(|v| (v.name.clone(), v))
            .collect();
        let authors: HashMap<String, BibliographyAuthor> = BibliographyAuthor::register_vec_tx(
            book_search_result_list
                .iter()
                .filter_map(|v| {
                    Some(v.authors.clone().into_iter().map(|v| BibliographyAuthor {
                        id: 0,
                        name: v,
                        memo: None,
                    }))
                })
                .flatten()
                .collect::<Vec<BibliographyAuthor>>()
                .as_slice(),
            &mut tx,
            true,
        )
            .await?
            .into_iter()
            .map(|v| (v.name.clone(), v))
            .collect();
        let bibliographies = Bibliography::register_vec_tx(
            book_search_result_list
                .iter()
                .enumerate()
                .map(|(i, v)| Bibliography {
                    id: 0,
                    isbn: v.isbn.clone(),
                    url: v.url.clone(),
                    title: v.title.clone(),
                    detail: v.detail.clone(),
                    authors: vec![],
                    publisher: get_optional(&publishers, &v.publisher),
                    publication_date: v.publication_date.clone(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    tmp_registration_id: Some(i),
                })
                .collect::<Vec<Bibliography>>()
                .as_slice(),
            &mut tx,
            true,
        )
            .await?;
        RelBibliographyAuthor::register_vec_tx(
            bibliographies
                .iter()
                .filter_map(|b| {
                    Some(
                        book_search_result_list[b.tmp_registration_id?]
                            .authors
                            .iter()
                            .filter_map(|a| {
                                Some(RelBibliographyAuthor {
                                    id: 0,
                                    bibliography_id: b.id,
                                    bibliography_author_id: authors.get(a)?.id,
                                })
                            }),
                    )
                })
                .flatten()
                .collect::<Vec<RelBibliographyAuthor>>()
                .as_slice(),
            &mut tx,
            true,
        )
            .await?;
        tx.commit().await?;
        Ok(bibliographies)
    }
}
