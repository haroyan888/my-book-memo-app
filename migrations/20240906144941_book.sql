-- Add migration script here

CREATE TABLE IF NOT EXISTS books (
	isbn_13 		CHAR(13) PRIMARY KEY,
	title			TEXT NOT NULL,
	publisher		TEXT NOT NULL,
	published_date	TEXT NOT NULL,
	description 	TEXT,
    image_url       TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS authors (
	isbn_13			CHAR(13) NOT NULL,
	author_name	TEXT NOT NULL
);