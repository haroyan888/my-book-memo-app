-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id          CHAR(36) PRIMARY KEY NOT NULL ,
    username    VARCHAR(128) NOT NULL UNIQUE ,
    password    VARCHAR(256) NOT NULL
);