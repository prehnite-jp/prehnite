task-category-foreshadowing = Foreshadowing
task-category-unexplained = Unexplained
task-template-recover = Recover foreshadowing.
task-template-recover-detail = Foreshadowing has been set up, but not yet resolved.
task-template-will-explain = Explain details.
task-template-will-explain-detail = Content that needs to be explained, but has not yet been explained.
book-search-api-example-name = Bibliographic Information Search API Configuration Example - Do Not Use
book-search-api-example-detail = API Response (Example):
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
    Writing Method
        - The function name must be mapper.
        - The function has arguments (isbn, search_text, response).
            isbn: ISBN used for ISBN search,
            search_text: String used for text search,
            response: API response object,
        - The function must return the following object:
            BookSearchResult[]
        - The return object can be constructed using the following function:
            fn new_rs(
                isbn: Option<String>,
                url: Option<String>,
                title: String,
                detail: Option<String>,
                authors: Option<Vec<String>>,
                publisher: Option<String>,
                publication_date: Option<NaiveDate>
            ) -> BookSearchResult
