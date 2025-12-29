use crate::db::app_global_schema::book_search_result::BookSearchResult;
use reqwest::IntoUrl;
use rhai::{Dynamic, Engine, EvalAltResult, Scope};
use sqlx::{Acquire, FromRow};

#[derive(Debug)]
pub enum BookSearchApiError {
    RequestError(reqwest::Error),
    ScriptError(Box<EvalAltResult>),
    ScriptParseError(rhai::ParseError),
    CustomError(String),
}

#[derive(Default, Clone, FromRow)]
pub struct BookSearchApi {
    pub id: i64,
    pub name: String,
    pub detail: String,
    pub isbn_url: String,
    pub text_url: String,
    pub mapping_script: String,
    pub is_example: bool,
}

fn option_str_to_dynamic(value: Option<String>) -> Dynamic {
    match value {
        None => Dynamic::from(()),
        Some(v) => Dynamic::from(v),
    }
}

impl BookSearchApi {
    async fn api_request(
        &self,
        url: impl IntoUrl,
        isbn: Option<String>,
        search_text: Option<String>,
    ) -> Result<Vec<BookSearchResult>, BookSearchApiError> {
        if self.is_example {
            return Ok(vec![BookSearchResult::example()]);
        }
        let response = match match reqwest::Client::new().get(url).send().await {
            Ok(v) => v,
            Err(e) => return Err(BookSearchApiError::RequestError(e)),
        }
        .json::<Dynamic>()
        .await
        {
            Ok(v) => v,
            Err(e) => return Err(BookSearchApiError::RequestError(e)),
        };
        self.mapper(
            response,
            option_str_to_dynamic(isbn),
            option_str_to_dynamic(search_text),
        )
    }

    fn mapper(
        &self,
        response: Dynamic,
        isbn: Dynamic,
        search_text: Dynamic,
    ) -> Result<Vec<BookSearchResult>, BookSearchApiError> {
        let mut engine = Engine::new();
        engine
            .register_type_with_name::<BookSearchResult>("BookSearchResult")
            .register_fn("new_res", BookSearchResult::new);
        let engine = engine;
        let ast = match engine.compile(self.mapping_script.clone()) {
            Ok(v) => v,
            Err(e) => return Err(BookSearchApiError::ScriptParseError(e)),
        };
        let mut scope = Scope::new();
        match engine.call_fn::<Dynamic>(&mut scope, &ast, "mapper", (isbn, search_text, response)) {
            Ok(v) => Ok(match v.into_typed_array::<BookSearchResult>() {
                Ok(v) => v,
                Err(e) => return Err(BookSearchApiError::CustomError(e.into())),
            }),
            Err(e) => Err(BookSearchApiError::ScriptError(e)),
        }
    }

    pub async fn search_isbn(
        &self,
        isbn: impl AsRef<str>,
    ) -> Result<Vec<BookSearchResult>, BookSearchApiError> {
        self.api_request(
            self.isbn_url.replace("<isbn>", isbn.as_ref()),
            Some(isbn.as_ref().into()),
            None,
        )
        .await
    }

    pub async fn search_text(
        &self,
        text: impl AsRef<str>,
    ) -> Result<Vec<BookSearchResult>, BookSearchApiError> {
        self.api_request(
            self.text_url.replace("<text>", text.as_ref()),
            None,
            Some(text.as_ref().into()),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::db::app_global_schema::book_search_api::BookSearchApi;
    use crate::db::app_global_schema::book_search_result::BookSearchResult;
    use rhai::Dynamic;
    use serde::de::IntoDeserializer;
    use serde::{Deserialize, Deserializer, Serialize};

    #[derive(Serialize, Clone)]
    struct Result {
        isbn: Option<String>,
        url: Option<String>,
        title: String,
        authors: Option<Vec<String>>,
        detail: Option<String>,
        publisher: Option<String>,
        publication_date: Option<String>,
    }

    #[derive(Serialize, Clone)]
    struct Object {
        status: i64,
        result: Vec<Result>,
    }

    const MAPPER_TEST_SCRIPT: &str = r#"fn mapper(isbn, search_text, response){
    let x = [];
    for result in response.result {
        x += new_res(
            result.isbn, // isbn
            result.url, // url
            result.title, // title
            result.detail, // detail
            result.authors, // authors
            result.publisher, // publisher
            result.publication_date, // publication date
        )
    }
    x
}"#;

    const MAPPER_OPTIONAL_TEST_SCRIPT: &str = r#"fn mapper(isbn, search_text, response){
    let x = [];
    x += new_res(
        (), // isbn
        (), // url
        "", // title
        (), // detail
        (), // authors
        (), // publisher
        (), // publication date
    );
    x
}"#;

    const MAPPER_REQUIRED_TEST_SCRIPT: &str = r#"fn mapper(isbn, search_text, response){
    let x = [];
    x += new_res(
        (), // isbn
        (), // url
        (), // title
        (), // detail
        (), // authors
        (), // publisher
        (), // publication date
    );
    x
}"#;

    #[test]
    #[should_panic]
    fn invalid_mapper_required_attr() {
        BookSearchApi {
            mapping_script: MAPPER_REQUIRED_TEST_SCRIPT.to_string(),
            ..Default::default()
        }
        .mapper(Default::default(), Default::default(), Default::default())
        .unwrap();
    }

    #[test]
    fn valid_mapper_optional_attr() {
        assert_eq!(
            BookSearchApi {
                mapping_script: MAPPER_OPTIONAL_TEST_SCRIPT.to_string(),
                ..Default::default()
            }
            .mapper(Default::default(), Default::default(), Default::default())
            .unwrap(),
            vec![BookSearchResult {
                isbn: None,
                url: None,
                title: "".to_string(),
                detail: None,
                authors: vec![],
                publisher: None,
                publication_date: None,
            }]
        );
    }

    #[test]
    fn valid_mapper() {
        let response = Object {
            status: 0,
            result: vec![
                Result {
                    isbn: Some("aaaaaaaa".to_string()),
                    url: None,
                    title: "bbbbbbbb".to_string(),
                    authors: Some(vec!["cccccccc".to_string(), "dddddddd".to_string()]),
                    detail: Some("eeeeeeee".to_string()),
                    publisher: None,
                    publication_date: Some("2023-01-01".to_string()),
                },
                Result {
                    isbn: None,
                    url: Some("ffffffff".to_string()),
                    title: "gggggggg".to_string(),
                    authors: None,
                    detail: None,
                    publisher: Some("hhhhhhhh".to_string()),
                    publication_date: None,
                },
                Result {
                    isbn: Some("iiiiiiii".to_string()),
                    url: Some("jjjjjjjj".to_string()),
                    title: "kkkkkkkk".to_string(),
                    authors: Some(vec!["llllllll".to_string()]),
                    detail: Some("mmmmmmmm".to_string()),
                    publisher: Some("nnnnnnnn".to_string()),
                    publication_date: Some("2023-05-10".to_string()),
                },
                Result {
                    isbn: Some("oooooooo".to_string()),
                    url: None,
                    title: "pppppppp".to_string(),
                    authors: Some(vec!["qqqqqqqq".to_string()]),
                    detail: None,
                    publisher: None,
                    publication_date: Some("2022-12-25".to_string()),
                },
                Result {
                    isbn: None,
                    url: Some("rrrrrrrr".to_string()),
                    title: "ssssssss".to_string(),
                    authors: None,
                    detail: Some("tttttttt".to_string()),
                    publisher: Some("uuuuuuuu".to_string()),
                    publication_date: None,
                },
                Result {
                    isbn: Some("vvvvvvvv".to_string()),
                    url: Some("wwwwwwww".to_string()),
                    title: "xxxxxxxx".to_string(),
                    authors: Some(vec!["yyyyyyyy".to_string(), "zzzzzzzz".to_string()]),
                    detail: Some("aaaaaaaa".to_string()),
                    publisher: None,
                    publication_date: Some("2021-07-07".to_string()),
                },
                Result {
                    isbn: None,
                    url: None,
                    title: "bbbbbbbb".to_string(),
                    authors: None,
                    detail: None,
                    publisher: Some("cccccccc".to_string()),
                    publication_date: None,
                },
                Result {
                    isbn: Some("dddddddd".to_string()),
                    url: Some("eeeeeeee".to_string()),
                    title: "ffffffff".to_string(),
                    authors: Some(vec!["gggggggg".to_string()]),
                    detail: Some("hhhhhhhh".to_string()),
                    publisher: Some("iiiiiiii".to_string()),
                    publication_date: Some("2020-01-01".to_string()),
                },
                Result {
                    isbn: Some("jjjjjjjj".to_string()),
                    url: None,
                    title: "kkkkkkkk".to_string(),
                    authors: None,
                    detail: Some("llllllll".to_string()),
                    publisher: None,
                    publication_date: Some("2024-02-29".to_string()),
                },
                Result {
                    isbn: None,
                    url: Some("mmmmmmmm".to_string()),
                    title: "nnnnnnnn".to_string(),
                    authors: Some(vec!["oooooooo".to_string()]),
                    detail: None,
                    publisher: Some("pppppppp".to_string()),
                    publication_date: None,
                },
            ],
        };

        let mapped_result: Vec<BookSearchResult> = response
            .clone()
            .result
            .into_iter()
            .map(|v| BookSearchResult {
                isbn: v.isbn,
                url: v.url,
                title: v.title,
                detail: v.detail,
                authors: v.authors.unwrap_or_default(),
                publisher: v.publisher,
                publication_date: v.publication_date,
            })
            .collect();

        let api = BookSearchApi {
            mapping_script: MAPPER_TEST_SCRIPT.to_string(),
            ..Default::default()
        };

        assert_eq!(
            api.mapper(
                rhai::serde::to_dynamic(response).unwrap(),
                Dynamic::from(()),
                Dynamic::from(())
            )
            .unwrap(),
            mapped_result
        )
    }
}
