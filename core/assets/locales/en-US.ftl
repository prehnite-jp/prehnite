task-category-foreshadowing = Foreshadowing
task-category-unexplained = Unexplained
task-template-recover = Recover foreshadowing.
task-template-recover-detail = Foreshadowing has been set up, but not yet resolved.
task-template-will-explain = Explain details.
task-template-will-explain-detail = Content that needs to be explained, but has not yet been explained.
book-search-api-example-name = Bibliographic Information Search API Configuration Example - Do Not Use
book-search-api-example-detail = API Response (Example):
    {"  {"}
        status: number,
        result: {"{"}
            isbn: string,
            title: string,
            authors: string[],
            detail: string,
            publication_date: string
    {"      }"}[]
    {"  }"}
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
                publication_date: Option<String>
            ) -> BookSearchResult
wip = WIP
main-content-placeholder = Enter the text here.
open-file = Open
new-file = New
error = Error
book-open-error = The book could not be opened. Please see the log file for more information.
permission-denied = Permission denied.
file-notfound = File does not exist.
cant-connect-database = A database connection could not be established.
now-loading = Now loading...
task = Task
item-no-select = item is not selected.
headline = Headline
paragraph = Paragraph
edit = Edit
draft = Draft
accepted-draft = Accepted draft
close-file = Close file
file = File
settings = Settings
show = Show
background-info-editor = Background info Editor
bibliography-editor = Bibliography Editor
help = Help
version-info = Version info
version-info-detail = {$app-name} v{$version}
exit = Exit
close = Close
settings_category_general = General
settings_entry_locale = Locale
settings_entry_font = Font
settings_entry_auto-open-last-opened-file = Automatically open the last opened file
unknown=unknown
apply=Apply
cancel=Cancel
search=Search
license-info=License Information
home=Home
license-info_message=Welcome to the License Information Viewer!
    Here you can check the license of this software and the licenses of dependent third-party software.
    License information is collected automatically and may be inaccurate. If this happens, please report it via GitHub Issues.
info=Information