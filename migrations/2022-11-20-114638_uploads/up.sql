-- Your SQL goes here
CREATE TABLE uploads
(
    id         uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    data       bytea            NOT NULL,
    date_added timestamp        NOT NULL        DEFAULT CURRENT_TIMESTAMP,
    mime_type  text,
    file_name  text
)
