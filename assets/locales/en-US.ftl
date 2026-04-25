task_category_foreshadowing = Foreshadowing
task_category_unexplained = Unexplained
task_template_recover = Recover foreshadowing.
task_template_recover_detail = Foreshadowing has been set up, but not yet resolved.
task_template_will_explain = Explain details.
task_template_will_explain_detail = Content that needs to be explained, but has not yet been explained.
book_search_api_example_name = Bibliographic Information Search API Configuration Example _ Do Not Use
book_search_api_example_detail = API Response (Example):
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
        _ The function name must be mapper.
        _ The function has arguments (isbn, search_text, response).
            isbn: ISBN used for ISBN search,
            search_text: String used for text search,
            response: API response object,
        _ The function must return the following object:
            BookSearchResult[]
        _ The return object can be constructed using the following function:
            fn new_rs(
                isbn: Option<String>,
                url: Option<String>,
                title: String,
                detail: Option<String>,
                authors: Option<Vec<String>>,
                publisher: Option<String>,
                publication_date: Option<String>
            ) _> BookSearchResult
wip = WIP
main_content_placeholder = Enter the text here.
open_file = Open
new_file = New
error = Error
book_open_error = The book could not be opened. Please see the log file for more information.
permission_denied = Permission denied.
file_notfound = File does not exist.
cant_connect_database = A database connection could not be established.
now_loading = Now loading...
task = Task
item_no_select = item is not selected.
headline = Headline
paragraph = Paragraph
edit = Edit
draft = Draft
accepted_draft = Accepted draft
close_file = Close file
file = File
settings = Settings
show = Show
background_info_editor = Background info Editor
bibliography_editor = Bibliography Editor
help = Help
version_info = Version info
version_info_detail = {$app_name} v{$version}
exit = Exit
close = Close
settings_category_general = General
settings_entry_locale = Locale
settings_entry_font = Font
settings_entry_auto_open_last_opened_file = Automatically open the last opened file
unknown=unknown
apply=Apply
cancel=Cancel
search=Search
license_info=License Information
home=Home
license_info_message=Welcome to the License Information Viewer!
    Here you can check the license of this software and the licenses of dependent third_party software.
    License information is collected automatically and may be inaccurate. If this happens, please report it via GitHub Issues.
info=Information
package_name=Name
package_authors=Authors
package_homepage=Homepage
package_repository=Repository
package_license=License