use crate::db::schema::{Bibliography, BibliographyAuthor, Publisher, RelBibliographyAuthor};
use crate::db::util::cushion_types::{OptionString, VecString};
use crate::db::util::get_optional;
use crate::to_hash_map_key_name;
use rhai::{CustomType, Dynamic, EvalAltResult, Position, TypeBuilder};
use sqlx::{Acquire, SqliteConnection};
use std::collections::HashMap;

#[derive(Default, Clone, CustomType, Debug, PartialEq)]
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

    fn publishers_from_bsr(bsr: &[BookSearchResult]) -> Vec<Publisher> {
        bsr.iter()
            .filter_map(|v| {
                Some(Publisher {
                    name: v.publisher.clone()?,
                    ..Default::default()
                })
            })
            .collect()
    }

    fn authors_from_bsr(bsr: &[BookSearchResult]) -> Vec<BibliographyAuthor> {
        bsr.iter()
            .filter_map(|v| {
                Some(v.authors.clone().into_iter().map(|v| BibliographyAuthor {
                    name: v,
                    ..Default::default()
                }))
            })
            .flatten()
            .collect()
    }

    fn bibliographies_from_bsr(
        bsr: &[BookSearchResult],
        publishers: &HashMap<String, Publisher>,
    ) -> Vec<Bibliography> {
        bsr.iter()
            .enumerate()
            .map(|(i, v)| Bibliography {
                isbn: v.isbn.clone(),
                url: v.url.clone(),
                title: v.title.clone(),
                detail: v.detail.clone(),
                publisher: get_optional(publishers, &v.publisher),
                publication_date: v.publication_date.clone(),
                tmp_registration_id: Some(i),
                ..Default::default()
            })
            .collect()
    }

    fn rel_bibliography_author_from_bibliographies(
        bsr: &[BookSearchResult],
        bibliographies: &[Bibliography],
        authors: &HashMap<String, BibliographyAuthor>,
    ) -> Vec<RelBibliographyAuthor> {
        bibliographies
            .iter()
            .filter_map(|b| {
                Some(bsr[b.tmp_registration_id?].authors.iter().filter_map(|a| {
                    Some(RelBibliographyAuthor {
                        id: 0,
                        bibliography_id: b.id,
                        bibliography_author_id: authors.get(a)?.id,
                    })
                }))
            })
            .flatten()
            .collect()
    }

    pub async fn register(
        conn: &mut SqliteConnection,
        book_search_result_list: Vec<BookSearchResult>,
    ) -> Result<Vec<Bibliography>, sqlx::Error> {
        let mut tx = conn.begin().await?;
        let publishers: HashMap<String, Publisher> = to_hash_map_key_name!(
            Publisher::register_vec(
                Self::publishers_from_bsr(book_search_result_list.as_slice()).as_slice(),
                &mut *tx,
                true,
            )
            .await?
        );
        let authors: HashMap<String, BibliographyAuthor> = to_hash_map_key_name!(
            BibliographyAuthor::register_vec(
                Self::authors_from_bsr(book_search_result_list.as_slice()).as_slice(),
                &mut *tx,
                true,
            )
            .await?
        );
        let bibliographies = Bibliography::register_vec(
            Self::bibliographies_from_bsr(book_search_result_list.as_slice(), &publishers)
                .as_slice(),
            &mut *tx,
            true,
        )
        .await?;
        RelBibliographyAuthor::register_vec(
            Self::rel_bibliography_author_from_bibliographies(
                book_search_result_list.as_slice(),
                bibliographies.as_slice(),
                &authors,
            )
            .as_slice(),
            &mut *tx,
            true,
        )
        .await?;
        tx.commit().await?;
        Ok(bibliographies)
    }
}
