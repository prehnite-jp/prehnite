CREATE TABLE metadata
(
    name  TEXT NOT NULL UNIQUE CHECK ( name <> '' ),
    value TEXT
);

