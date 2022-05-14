-- Your SQL goes here
CREATE TABLE users
(
    email    VARCHAR(255) NOT NULL PRIMARY KEY UNIQUE,
    password VARCHAR(255) NOT NULL
);
CREATE TABLE posts
(
    slug             VARCHAR(300) PRIMARY KEY,
    title            VARCHAR  NOT NULL,
    content          TEXT     NOT NULL,
    rendered_content TEXT,
    published        BOOLEAN  NOT NULL DEFAULT false,
    created_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags             VARCHAR(300)
)