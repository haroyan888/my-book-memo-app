CREATE TABLE IF NOT EXISTS memo (
    id          CHAR(36) PRIMARY KEY,
    isbn_13		CHAR(13) NOT NULL,
    text		TEXT NOT NULL
);