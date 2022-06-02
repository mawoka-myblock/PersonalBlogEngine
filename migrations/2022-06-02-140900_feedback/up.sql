-- Your SQL goes here

ALTER TABLE posts ADD id uuid NOT NULL UNIQUE DEFAULT gen_random_uuid();
ALTER TABLE posts ALTER COLUMN slug TYPE text;

CREATE TABLE feedback (
    id uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    ip_hash bytea UNIQUE NOT NULL,
    feedback_text text NULL,
    thumbs_up boolean NOT NULL,
    post_id uuid NOT NULL REFERENCES posts(id) ON DELETE CASCADE
);
