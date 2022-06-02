-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS feedback;
ALTER TABLE posts DROP COLUMN id;
ALTER TABLE posts
    ALTER COLUMN slug TYPE varchar(300);
ALTER TABLE posts
    DROP COLUMN thumbs_up;
ALTER TABLE posts
    DROP COLUMN thumbs_down;