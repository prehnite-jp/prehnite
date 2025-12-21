CREATE TABLE settings
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    setting_key   TEXT NOT NULL UNIQUE,
    setting_value TEXT
);

-- タスクのカテゴリ
CREATE TABLE global_default_task_categories
(
    id                          INTEGER PRIMARY KEY AUTOINCREMENT,
    name                        TEXT    NOT NULL,
    -- autocomplete_paragraph_link: 段落リンクに紐づけたとき、自動的に完了としてマークされます。
    autocomplete_paragraph_link INTEGER NOT NULL DEFAULT 0 CHECK (autocomplete_paragraph_link = 0 /* false */ OR
                                                                  autocomplete_paragraph_link = 1 /* true */)
);

CREATE TABLE global_default_task_templates
(
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    task_category_id INTEGER REFERENCES global_default_task_categories (id), -- タスクカテゴリ。
    title            TEXT NOT NULL,
    detail           TEXT
);

CREATE TABLE global_default_bibliographies
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    isbn       TEXT,
    url        TEXT,
    title      TEXT    NOT NULL,
    detail     TEXT,
    author     TEXT,
    created_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP
);
