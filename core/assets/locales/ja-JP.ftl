task-category-foreshadowing = 伏線
task-category-unexplained = 未解説
task-template-recover = 伏線を回収する。
task-template-recover-detail = 伏線を立てましたが、まだ回収されていません。
task-template-will-explain = 詳細を解説する。
task-template-will-explain-detail = 解説する必要がある内容ですが、まだ解説されていません。
book-search-api-example-name =　書誌情報検索API設定例 - 使用禁止
book-search-api-example-detail = APIのレスポンス(例):
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
                publication_date: Option<String>
            ) -> BookSearchResult
wip = 作業中
main-content-placeholder = ここに本文を入力。
open-file = 開く
new-file = 新規作成
error = エラー
book-open-error = ブックを開けませんでした。詳細はログファイルを確認してください。
permission-denied = ファイルへのアクセスが拒否されました。
file-notfound = ファイルが存在しません。
cant-connect-database = データベース接続が確立できませんでした。
now-loading = 読み込み中...
task = タスク
item-no-select = アイテムは選択されていません。
headline = 見出し
paragraph = 段落
edit = 編集
draft = 下書き
accepted-draft = 採択された下書き
close-file = ファイルを閉じる
file = ファイル
settings = 設定
show = 表示
background-info-editor = 背景情報エディタ
bibliography-editor = 参考文献エディタ
help = ヘルプ
version-info = バージョン情報
version-info-detail = {$app-name} v{$version}
exit = 終了
close = 閉じる
settings_category_general = 全般
settings_entry_locale = 言語と地域
settings_entry_font = フォント
settings_entry_auto-open-last-opened-file = 最後に開いたファイルを自動で開く
unknown=不明
apply=適用
cancel=キャンセル
search=検索