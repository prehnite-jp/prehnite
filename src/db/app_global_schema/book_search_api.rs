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
