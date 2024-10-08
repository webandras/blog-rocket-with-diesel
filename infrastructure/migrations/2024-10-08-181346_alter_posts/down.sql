ALTER TABLE posts
    DROP COLUMN created_at,
    DROP COLUMN updated_at;

-- DROP FUNCTION IF EXISTS on_posts_update;
DROP TRIGGER update_posts_updated_at
    ON posts;