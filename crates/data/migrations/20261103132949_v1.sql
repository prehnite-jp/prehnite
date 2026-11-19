CREATE TABLE metadata -- Metadata
(
    name  TEXT NOT NULL UNIQUE CHECK ( name <> '' ),
    value TEXT
);

CREATE TABLE documents
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    title      BLOB      NOT NULL,
    content    BLOB      NOT NULL,
    deleted    BOOLEAN   NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TRIGGER document_delete_guard
    BEFORE DELETE
    ON documents
    WHEN NOT OLD.deleted
BEGIN
    SELECT RAISE(ABORT, 'The item cannot be physically deleted because it has not been logically deleted.');
END;

CREATE TABLE document_histories
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id INTEGER   NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    title       BLOB,
    content     BLOB      NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL
);

CREATE TRIGGER document_auto_vcs
    AFTER UPDATE
    ON documents
    WHEN NEW.title <> OLD.title OR NEW.content <> OLD.content
BEGIN
    INSERT INTO document_histories(document_id, title, content, created_at, updated_at)
    VALUES (OLD.id, OLD.title, OLD.content, OLD.created_at, OLD.updated_at);
END;

CREATE TABLE deleted_documents
(
    document_id INTEGER PRIMARY KEY,
    deleted_at  TIMESTAMP NOT NULL
);

CREATE TRIGGER document_delete_history
    AFTER DELETE
    ON documents
BEGIN
    INSERT INTO deleted_documents(document_id, deleted_at) VALUES (OLD.id, strftime('%FT%H:%M:%fZ', 'now'));
END;

CREATE TABLE tags
(
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    title    TEXT NOT NULL UNIQUE,
    details  TEXT,
    metadata TEXT NOT NULL
);

CREATE TABLE rel_docs_and_tags
(
    document_id INTEGER NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    tag_id      INTEGER NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    UNIQUE (document_id, tag_id)
);

CREATE TABLE rel_docs_and_background_info
(
    document_id         INTEGER NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    bg_info_document_id INTEGER NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    UNIQUE (document_id, bg_info_document_id)
);

CREATE TABLE binders
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    title      TEXT NOT NULL,
    details    TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE bound_documents
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    binder_id   INTEGER NOT NULL REFERENCES binders (id) ON DELETE CASCADE,
    document_id INTEGER NOT NULL REFERENCES documents (id) ON DELETE CASCADE,
    next        INTEGER REFERENCES bound_documents (id) ON DELETE SET NULL,
    prev        INTEGER REFERENCES bound_documents (id) ON DELETE SET NULL,
    UNIQUE (binder_id, document_id)
);