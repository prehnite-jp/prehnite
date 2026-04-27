task_category_foreshadowing = 伏線
task_category_unexplained = 未解説
task_template_recover = 伏線を回収する。
task_template_recover_detail = 伏線を立てましたが、まだ回収されていません。
task_template_will_explain = 詳細を解説する。
task_template_will_explain_detail = 解説する必要がある内容ですが、まだ解説されていません。
book_search_api_example_name =　書誌情報検索API設定例 _ 使用禁止
book_search_api_example_detail = APIのレスポンス(例):
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
    書き方
        _ 関数名はmapperでなければならない。
        _ 関数は引数(isbn, search_text, response)を持つ。
            isbn: ISBN検索に使用したISBN,
            search_text: テキスト検索に使用した文字列,
            response: APIレスポンスのオブジェクト,
        _ 関数は以下のオブジェクトを返さなければならない。
            BookSearchResult[]
        _ 戻り値用のオブジェクトは以下の関数で構築できます。
            fn new_rs(
                isbn: Option<String>,
                url: Option<String>,
                title: String,
                detail: Option<String>,
                authors: Option<Vec<String>>,
                publisher: Option<String>,
                publication_date: Option<String>
            ) _> BookSearchResult
wip = 作業中
main_content_placeholder = ここに本文を入力。
open_file = 開く
new_file = 新規作成
error = エラー
book_open_error = ブックを開けませんでした。詳細はログファイルを確認してください。
permission_denied = ファイルへのアクセスが拒否されました。
file_notfound = ファイルが存在しません。
cant_connect_database = データベース接続が確立できませんでした。
now_loading = 読み込み中...
task = タスク
item_no_select = アイテムは選択されていません。
headline = 見出し
paragraph = 段落
edit = 編集
draft = 下書き
accepted_draft = 採択された下書き
close_file = ファイルを閉じる
file = ファイル
settings = 設定
show = 表示
background_info_editor = 背景情報エディタ
bibliography_editor = 参考文献エディタ
help = ヘルプ
version_info = バージョン情報
version_info_detail = {$app_name} v{$version}
exit = 終了
close = 閉じる
settings_category_general = 全般
settings_entry_locale = 言語と地域
settings_entry_font = フォント
settings_entry_auto_open_last_opened_file = 最後に開いたファイルを自動で開く
unknown=不明
apply=適用
cancel=キャンセル
search=検索
license_info=ライセンス情報
home=ホーム
license_info_message=ライセンス情報表示へようこそ！
    ここでは本ソフトウェアのライセンスと依存関係の第三者ソフトウェアのライセンスを確認できます。
    ライセンス情報は自動収集です。不正確な場合があります。そのような場合はGitHub Issuesよりご報告ください。
info=情報
package_name=パッケージ名
package_authors=作者
package_homepage=ホームページ
package_repository=リポジトリ
package_license=ライセンス
new_parent_headline = 親見出し
new_headline = 子見出し
new_paragraph = 段落
no_title = 無題
not_opened = ブックは開かれていません。
en-US = English (US)
ja-JP = 日本語
confirm = 確認
confirm_settings_not_applied = 設定は変更されていますが、保存されていません。
    設定を保存しますか？
settings_entry_theme = テーマ
dark = ダーク
light = ライト