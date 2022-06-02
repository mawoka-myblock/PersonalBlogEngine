-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS feedback;
ALTER TABLE posts DROP COLUMN id;