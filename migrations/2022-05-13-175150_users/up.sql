-- Your SQL goes here
CREATE TABLE users
(
    email    VARCHAR(255) NOT NULL PRIMARY KEY UNIQUE,
    password VARCHAR(255) NOT NULL
)