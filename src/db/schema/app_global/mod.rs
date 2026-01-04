pub mod book_search_api;
pub mod book_search_result;

use crate::db::schema::app_global::book_search_api::BookSearchApi;
use sqlx::{Acquire, FromRow, SqliteConnection, SqliteTransaction};

pub type AppGlobalDefaultTaskCategory = crate::db::schema::TaskCategory;

pub type AppGlobalDefaultTaskTemplate = crate::db::schema::TaskTemplate;

pub type AppGlobalDefaultTag = crate::db::schema::Tag;

pub type AppGlobalDefaultPublisher = crate::db::schema::Publisher;

pub type AppGlobalDefaultBibliography = crate::db::schema::Bibliography;

pub type AppGlobalDefaultBibliographyAuthor = crate::db::schema::BibliographyAuthor;

pub type AppGlobalDefaultRelBibliographyAuthor = crate::db::schema::RelBibliographyAuthor;

async fn register_default_data_task_categories(
    tx: &mut SqliteTransaction<'_>,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO task_categories(id, name, autocomplete_paragraph_link) VALUES (?,?,?), (?,?,?)")
        .bind(1)
        .bind("伏線".to_string())
        .bind(true)
        .bind(2)
        .bind("未解説".to_string())
        .bind(true)
        .execute(&mut **tx).await?;
    Ok(())
}

pub async fn register_default_data_task_category_and_templates(
    tx: &mut SqliteTransaction<'_>,
) -> Result<(), sqlx::Error> {
    register_default_data_task_categories(tx).await?;
    let values = vec![
        AppGlobalDefaultTaskTemplate {
            id: 1,
            task_category: Some(AppGlobalDefaultTaskCategory {
                id: 1,
                ..Default::default()
            }),
            title: "伏線を回収する。".to_string(),
            detail: Some("伏線を立てましたが、まだ回収されていません。".to_string()),
        },
        AppGlobalDefaultTaskTemplate {
            id: 2,
            task_category: Some(AppGlobalDefaultTaskCategory {
                id: 2,
                ..Default::default()
            }),
            title: "詳細を解説する。".to_string(),
            detail: Some("解説する必要がある内容ですが、まだ解説されていません。".to_string()),
        },
    ];
    AppGlobalDefaultTaskTemplate::register_vec_tx(values.as_slice(), tx, false).await?;
    Ok(())
}

pub async fn register_default_data_book_search_api(
    tx: &mut SqliteTransaction<'_>,
) -> Result<(), sqlx::Error> {
    BookSearchApi {
        id: 0,
        name: "Search API Setting Example. do not use.".to_string(),
        detail: "example API response: { status: number, result: { isbn: string, title: string, authors: string[], detail: string, publication_date: string }[] }
    How to write. 書き方
        - The function name must be a mapper. 関数名はmapperでなければならない。
        - The function has an argument response, which is an API response object.関数は引数(isbn, search_text, response)を持つ。
            isbn: The ISBN used for the ISBN search. ISBN検索に使用したISBN,
            search_text: The string used for the text search. テキスト検索に使用した文字列,
            response: The API response object. APIレスポンスのオブジェクト,
        - The function must return the following object. 関数は以下のオブジェクトを返さなければならない。
            BookSearchResult[]
        - The object can be constructed with the following functions. オブジェクトは以下の関数で構築できます。
            fn new_rs(isbn: Option<String>, url: Option<String>, title: String, detail: Option<String>, authors: Option<Vec<String>>, publisher: Option<String>, publication_date: Option<NaiveDate>) -> BookSearchResult".to_string(),
        isbn_url: "https://example.com/api/book?isbn=<isbn>".to_string(),
        text_url: "https://example.com/api/book?search=<text>".to_string(),
        mapping_script: r#"fn mapper(isbn, search_text, response){
    let x = [];
    for result in response.result {
        x += new_res(
            result.isbn, // isbn
            "", // url
            result.title, // title
            result.detail, // detail
            result.authors, // authors
            (), // publisher (Option::None)
            result.publication_date, // publication date
        )
    }
    x
}"#.to_string(),
        is_example: true,
    }.register_tx(tx, false).await?;
    Ok(())
}

pub async fn register_all_default_data(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    let mut tx = conn.begin().await?;
    register_default_data_task_category_and_templates(&mut tx).await?;
    register_default_data_book_search_api(&mut tx).await?;
    tx.commit().await
}

#[cfg(test)]
mod tests {
    use crate::db::schema::app_global::register_all_default_data;
    use sqlx::SqlitePool;

    #[sqlx::test(migrator = "crate::db::migrate::app_global::MIGRATOR")]
    async fn valid_register_all_default_data(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();
        register_all_default_data(&mut conn).await.unwrap();
    }
}
