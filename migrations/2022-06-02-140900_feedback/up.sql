-- Your SQL goes here

ALTER TABLE posts
    ADD id uuid NOT NULL UNIQUE DEFAULT gen_random_uuid();
ALTER TABLE posts
    ALTER COLUMN slug TYPE text;
ALTER TABLE posts
    ADD thumbs_up smallint NOT NULL DEFAULT 0;
ALTER TABLE posts
    ADD thumbs_down smallint NOT NULL DEFAULT 0;

CREATE TABLE feedback
(
    id            uuid PRIMARY KEY NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    ip_hash       bytea UNIQUE     NOT NULL,
    feedback_text text             NULL,
    thumbs_up     boolean          NOT NULL,
    post_id       uuid             NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    created_at    timestamp        NOT NULL        DEFAULT now()
);
