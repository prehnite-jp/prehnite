task-category-foreshadowing = 伏線
task-category-unexplained = 未解説
task-template-recover = 伏線を回収する。
task-template-recover-detail = 伏線を立てましたが、まだ回収されていません。
task-template-will-explain = 詳細を解説する。
task-template-will-explain-detail = 解説する必要がある内容ですが、まだ解説されていません。
book-search-api-example-name =　書誌情報検索API設定例 - 使用禁止
book-search-api-example-detail = APIのレスポンス(例):
    {
        status: number,
        result: {
            isbn: string,
            title: string,
            authors: string[],
            detail: string,
            publication_date: string
        }[]
    }
    書き方
        - 関数名はmapperでなければならない。
        - 関数は引数(isbn, search_text, response)を持つ。
            isbn: ISBN検索に使用したISBN,
            search_text: テキスト検索に使用した文字列,
            response: APIレスポンスのオブジェクト,
        - 関数は以下のオブジェクトを返さなければならない。
            BookSearchResult[]
        - 戻り値用のオブジェクトは以下の関数で構築できます。
            fn new_rs(
                isbn: Option<String>,
                url: Option<String>,
                title: String,
                detail: Option<String>,
                authors: Option<Vec<String>>,
                publisher: Option<String>,
                publication_date: Option<NaiveDate>
            ) -> BookSearchResult
