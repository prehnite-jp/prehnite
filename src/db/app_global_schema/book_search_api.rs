use crate::db::app_global_schema::BookSearchApi;
use crate::db::schema::Bibliography;
use crate::db::util::placeholder_helper;
use reqwest::IntoUrl;
use rhai::{Array, CustomType, Dynamic, Engine, EvalAltResult, Position, Scope, TypeBuilder};
use sqlx::{Acquire, SqliteConnection};

struct OptionString(Option<String>);

impl From<Option<String>> for OptionString {
    fn from(value: Option<String>) -> Self {
        OptionString(value)
    }
}

impl From<Dynamic> for OptionString {
    fn from(value: Dynamic) -> Self {
        if value.is_string() {
            Some(value.into_string().unwrap_or_default()).into()
        } else {
            None.into()
        }
    }
}

impl From<OptionString> for Option<String> {
    fn from(value: OptionString) -> Self {
        value.0
    }
}

struct VecString(Vec<String>);

impl From<Vec<String>> for VecString {
    fn from(value: Vec<String>) -> Self {
        VecString(value)
    }
}

impl From<Array> for VecString {
    fn from(value: Array) -> Self {
        value
            .into_iter()
            .map(|v| v.into_string().unwrap_or_default())
            .filter(|v| !v.is_empty())
            .collect::<Vec<String>>()
            .into()
    }
}

impl From<VecString> for Vec<String> {
    fn from(value: VecString) -> Self {
        value.0
    }
}

#[derive(Default, Clone, CustomType, Debug)]
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
    fn example() -> Self {
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
}

impl BookSearchResult {
    fn new(
        isbn: Dynamic,
        url: Dynamic,
        title: String,
        detail: Dynamic,
        authors: Array,
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

}

#[derive(Debug)]
pub enum BookSearchApiError {
    RequestError(reqwest::Error),
    ScriptError(Box<EvalAltResult>),
    ScriptParseError(rhai::ParseError),
    CustomError(String),
}

impl BookSearchApi {
    async fn api_request(
        &self,
        url: impl IntoUrl,
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
        self.mapper(response)
    }

    fn mapper(&self, response: Dynamic) -> Result<Vec<BookSearchResult>, BookSearchApiError> {
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
        match engine.call_fn::<Dynamic>(&mut scope, &ast, "mapper", (response,)) {
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
        self.api_request(self.isbn_url.replace("<isbn>", isbn.as_ref()))
            .await
    }

    pub async fn search_text(
        &self,
        text: impl AsRef<str>,
    ) -> Result<Vec<BookSearchResult>, BookSearchApiError> {
        self.api_request(self.text_url.replace("<text>", text.as_ref()))
            .await
    }
}
