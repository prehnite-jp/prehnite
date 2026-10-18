INSERT INTO book_search_api(id, name, detail, isbn_url, text_url, mapping_script, is_example)
VALUES (1,
        'example',
        'A example of book search api connector',
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
}',
        1);