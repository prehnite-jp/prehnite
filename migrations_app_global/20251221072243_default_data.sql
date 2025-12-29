INSERT INTO task_categories(id, name, autocomplete_paragraph_link)
VALUES (1, '伏線', 1),
       (2, '未解説', 1);

INSERT INTO task_templates(task_category_id, title, detail)
VALUES (1, '伏線を回収する。', '伏線を立てましたが、まだ回収されていません。'),
       (2, '詳細を解説する。', '解説する必要がある内容ですが、まだ解説されていません。');

INSERT INTO book_search_api(id, name, detail, isbn_url, text_url, mapping_script, is_example)
VALUES (1, 'Search API Setting Example. do not use.',
        'example API response: { status: number, result: { isbn: string, title: string, authors: string[], detail: string, publication_date: string }[] }
    How to write. 書き方
        - The function name must be a mapper. 関数名はmapperでなければならない。
        - The function has an argument response, which is an API response object.関数は引数(isbn, search_text, response)を持つ。
            isbn: The ISBN used for the ISBN search. ISBN検索に使用したISBN,
            search_text: The string used for the text search. テキスト検索に使用した文字列,
            response: The API response object. APIレスポンスのオブジェクト,
        - The function must return the following object. 関数は以下のオブジェクトを返さなければならない。
            BookSearchResult[]
        - The object can be constructed with the following functions. オブジェクトは以下の関数で構築できます。
            fn new_rs(isbn: Option<String>, url: Option<String>, title: String, detail: Option<String>, authors: Option<Vec<String>>, publisher: Option<String>, publication_date: Option<NaiveDate>) -> BookSearchResult',
        'https://example.com/api/book?isbn=<isbn>',
        'https://example.com/api/book?search=<text>',
        'fn mapper(isbn, search_text, response){
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
}', 1);
