CREATE TABLE settings
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    setting_key   TEXT NOT NULL UNIQUE,
    setting_value TEXT
);

-- タスクのカテゴリ
CREATE TABLE task_categories
(
    id                          INTEGER PRIMARY KEY AUTOINCREMENT,
    name                        TEXT    NOT NULL,
    -- autocomplete_paragraph_link: 段落リンクに紐づけたとき、自動的に完了としてマークされます。
    autocomplete_paragraph_link INTEGER NOT NULL DEFAULT 0 CHECK (autocomplete_paragraph_link = 0 /* false */ OR
                                                                  autocomplete_paragraph_link = 1 /* true */)
);

CREATE TABLE task_templates
(
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    task_category_id INTEGER REFERENCES task_categories (id), -- タスクカテゴリ。
    title            TEXT NOT NULL,
    detail           TEXT
);

CREATE VIEW view_deserializable_task_template AS
SELECT task_templates.*,
       task_categories.id                          AS tc_id,
       task_categories.name                        AS tc_name,
       task_categories.autocomplete_paragraph_link AS tc_autocomplete_paragraph_link
FROM task_templates
         LEFT OUTER JOIN task_categories
                         ON task_templates.task_category_id = task_categories.id;

-- 出版社
CREATE TABLE publishers
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    memo TEXT
);

-- 文献
CREATE TABLE bibliographies
(
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    isbn                TEXT,
    url                 TEXT,
    title               TEXT    NOT NULL,
    detail              TEXT,
    publisher_id        INTEGER REFERENCES publishers (id) ON DELETE SET NULL,
    publication_date    INTEGER,
    tmp_registration_id INTEGER,
    created_at          INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at          INTEGER NOT NULL DEFAULT (unixepoch())
);

-- 同姓同名は同一人物として扱う。あくまで索引である。
CREATE TABLE bibliography_authors
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    memo TEXT
);

CREATE TABLE rel_bibliography_authors
(
    id                     INTEGER PRIMARY KEY AUTOINCREMENT,
    bibliography_id        INTEGER NOT NULL REFERENCES bibliographies (id) ON DELETE CASCADE,
    bibliography_author_id INTEGER NOT NULL REFERENCES bibliography_authors (id) ON DELETE CASCADE,
    UNIQUE (bibliography_id, bibliography_author_id)
);

CREATE TRIGGER update_at_bibliographies
    AFTER
        UPDATE
    ON bibliographies
    FOR EACH ROW
BEGIN
    UPDATE bibliographies
    SET updated_at = (unixepoch())
    WHERE ROWID = NEW.ROWID;
END;

-- 書誌情報検索のAPI連携用
-- isbn検索のAPIが以下のような場合、<isbn>は検索対象isbnに置き換えられます。
-- url: https://api.books.example.com/search?isbn=<isbn>
-- url: https://api.books.example.com/<isbn>/info
-- タイトル・内容等による検索のAPIが以下のような場合、<text>は検索対象文字列に置き換えられます。
-- url: https://api.books.example.com/search?text=<text>
-- url: https://api.books.example.com/search/<text>/info
--
-- mapping_scriptではAPIのレスポンスをBibliographyにマッピングします。
-- mapping_scriptは[rhai](https://rhai.rs/book/engine/expressions.html)です。
CREATE TABLE book_search_api
(
    id             INTEGER PRIMARY KEY,
    name           TEXT    NOT NULL UNIQUE CHECK (name <> ''),
    detail         TEXT    NOT NULL DEFAULT (''),
    isbn_url       TEXT    NOT NULL,
    text_url       TEXT    NOT NULL,
    mapping_script TEXT    NOT NULL,
    is_example     INTEGER NOT NULL DEFAULT (0) CHECK (is_example = 0 /* false */ OR is_example = 1 /* true */)
);
