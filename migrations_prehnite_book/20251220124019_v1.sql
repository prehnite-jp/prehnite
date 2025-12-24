-- 背景情報
CREATE TABLE background_info
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    body       TEXT    NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
);

-- 背景情報の最終更新時刻を更新
CREATE TRIGGER update_at_background_info
    AFTER
        UPDATE
    ON background_info
    FOR EACH ROW
BEGIN
    UPDATE background_info
    SET updated_at = (unixepoch())
    WHERE ROWID = NEW.ROWID;
END;

-- タグ
CREATE TABLE tags
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

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

-- 文献の最終更新時刻を更新
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

-- アイテム
CREATE TABLE items
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    item_type  TEXT    NOT NULL CHECK ( item_type = 'headline' OR item_type = 'paragraph' ),
    title      TEXT    NOT NULL
);

-- アイテムタイプは読み取り専用属性なので変更を拒否
CREATE TRIGGER deny_change_item_type
    BEFORE UPDATE
    ON items
BEGIN
    SELECT RAISE(FAIL, 'column item_type is readonly.') WHERE OLD.item_type <> NEW.item_type;
END;

-- 見出し
CREATE TABLE headlines
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id      INTEGER NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    parent_id    INTEGER REFERENCES headlines (id) ON DELETE RESTRICT,
    headline_pos INTEGER, -- 同一の親間での子見出し同士での順序。小さいほど上、大きいほど下。 NULLの場合は最上。
    UNIQUE (item_id)
);

-- 見出しの挿入時、対応するアイテムのタイプが見出しでないなら拒否
CREATE TRIGGER deny_insert_headlines_item_is_other
    BEFORE INSERT
    ON headlines
BEGIN
    SELECT RAISE(FAIL, 'item type is not headline.')
    FROM items
    WHERE items.id = NEW.item_id
      AND items.item_type <> 'headline';
END;

-- アイテムIDは読み取り専用属性なので変更を拒否
CREATE TRIGGER deny_update_headlines_column_item_id
    BEFORE UPDATE
    ON headlines
BEGIN
    SELECT RAISE(FAIL, 'column item_id is readonly.') WHERE OLD.item_id <> NEW.item_id;
END;

-- 段落
CREATE TABLE paragraph
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id       INTEGER NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    headline_id   INTEGER NOT NULL REFERENCES headlines (id) ON DELETE CASCADE,
    paragraph_pos INTEGER, -- 段落の見出し内での位置。小さいほど上、大きいほど下。 NULLの場合は最上。
    UNIQUE (item_id),
    UNIQUE (headline_id, paragraph_pos)
);

-- 段落の下書き
CREATE TABLE draft
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    paragraph_id INTEGER NOT NULL REFERENCES paragraph (id) ON DELETE CASCADE,
    draft_pos    INTEGER, -- 段落の中での下書きの位置。小さいほど左、大きいほど右。 NULLの場合は最左。
    title        TEXT    NOT NULL,
    body         TEXT    NOT NULL,
    created_at   INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at   INTEGER NOT NULL DEFAULT (unixepoch()),
    UNIQUE (paragraph_id, draft_pos)
);

-- 下書きの最終更新時刻を更新
CREATE TRIGGER update_at_draft
    AFTER
        UPDATE
    ON draft
    FOR EACH ROW
BEGIN
    UPDATE draft SET updated_at = (unixepoch()) WHERE ROWID = NEW.ROWID;
END;

-- 段落に採用された下書きの列を追加 (相互参照)
ALTER TABLE paragraph
    ADD COLUMN accepted_draft_id INTEGER REFERENCES draft (id) ON DELETE RESTRICT;

/* 最終的な`paragraph`のスキーマ
CREATE TABLE paragraph
(
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id             INTEGER NOT NULL    REFERENCES items (id)       ON DELETE CASCADE,
    headline_id         INTEGER NOT NULL    REFERENCES headlines (id)   ON DELETE CASCADE,
    accepted_draft_id   INTEGER             REFERENCES draft(id)        ON DELETE RESTRICT,
    UNIQUE (item_id)
);
*/

-- 段落の挿入時点では採用済み下書きは存在しないので挿入を拒否
CREATE TRIGGER check_on_insert_paragraph_accepted_draft
    BEFORE INSERT
    ON paragraph
BEGIN
    SELECT RAISE(FAIL, 'draft is not acceptable') WHERE NEW.accepted_draft_id IS NOT NULL;
END;

-- 段落の更新時、採用済み下書きが段落に紐づいていることを保証
CREATE TRIGGER check_on_update_paragraph_accepted_draft
    BEFORE UPDATE
    ON paragraph
BEGIN
    -- 下書きが採用された場合、それが段落に紐づいているか確認
    SELECT RAISE(FAIL, 'draft is not acceptable')
    WHERE NEW.accepted_draft_id IS NOT NULL -- 採用された下書きがある
      AND NOT EXISTS (SELECT id
                      FROM draft
                      WHERE draft.id = NEW.accepted_draft_id
                        AND draft.paragraph_id = NEW.id); -- 下書きが段落に紐づいていない
END;

-- 段落の概要
CREATE TABLE paragraph_summaries
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    paragraph_id INTEGER NOT NULL REFERENCES paragraph (id) ON DELETE CASCADE,
    title        TEXT    NOT NULL,
    detail       TEXT    NOT NULL,
    created_at   INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at   INTEGER NOT NULL DEFAULT (unixepoch())
);

-- 下書きの最終更新時刻を更新
CREATE TRIGGER update_at_paragraph_summaries
    AFTER UPDATE
    ON paragraph_summaries
    FOR EACH ROW
BEGIN
    UPDATE paragraph_summaries SET updated_at = (unixepoch()) WHERE ROWID = NEW.ROWID;
END;

-- 背景情報の参考文献リスト
CREATE TABLE background_references_list
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    background_info_id INTEGER NOT NULL REFERENCES background_info (id) ON DELETE CASCADE,
    bibliography_id    INTEGER NOT NULL REFERENCES bibliographies (id) ON DELETE CASCADE,
    location           TEXT    NOT NULL -- ページ数・行数等
);

-- アイテム単位の参考文献リスト
CREATE TABLE item_references_list
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id         INTEGER NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    bibliography_id INTEGER NOT NULL REFERENCES bibliographies (id) ON DELETE CASCADE,
    location        TEXT    NOT NULL -- ページ数・行数等
);

-- アイテムに付与されているタグ
CREATE TABLE rel_tag_and_item
(
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    tag_id  INTEGER NOT NULL REFERENCES tags (id) ON DELETE CASCADE
);

-- 背景情報とアイテムの紐づけ
CREATE TABLE rel_background_and_item
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id            INTEGER NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    background_info_id INTEGER NOT NULL REFERENCES background_info (id) ON DELETE CASCADE
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

-- タスク
CREATE TABLE tasks
(
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id          INTEGER NOT NULL REFERENCES items (id) ON DELETE CASCADE,
    task_category_id INTEGER REFERENCES task_categories (id),
    title            TEXT    NOT NULL,
    detail           TEXT,
    is_finished      INTEGER NOT NULL DEFAULT 0 CHECK ( is_finished = 0 /* false */ OR is_finished = 1 /* true */ )
);

-- 段落間に置くリンク
-- BがAを参照する: from = B, to = A
CREATE TABLE paragraph_link
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    from_paragraph_id INTEGER NOT NULL REFERENCES paragraph (id) ON DELETE CASCADE,
    to_paragraph_id   INTEGER NOT NULL REFERENCES paragraph (id) ON DELETE CASCADE,
    task_id           INTEGER REFERENCES tasks (id) ON DELETE SET NULL,
    comment           TEXT,
    UNIQUE (from_paragraph_id, to_paragraph_id, task_id)
);

-- `autocomplete_paragraph_link`が有効の場合、完了としてマーク
CREATE TRIGGER autocomplete_paragraph_link_task
    AFTER UPDATE
    ON paragraph_link
    FOR EACH ROW
BEGIN
    UPDATE tasks
    SET is_finished = 1
    WHERE tasks.id = NEW.task_id
      AND (SELECT task_categories.autocomplete_paragraph_link
           FROM task_categories
           WHERE task_categories.id = tasks.task_category_id) = 1;
END;
