-- Your SQL goes here
CREATE TABLE posts
(
    slug             VARCHAR(300) PRIMARY KEY,
    title            VARCHAR NOT NULL,
    content          TEXT    NOT NULL,
    rendered_content TEXT,
    published        BOOLEAN NOT NULL DEFAULT false,
    created_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags             TEXT[]
);