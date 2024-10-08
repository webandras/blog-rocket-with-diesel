ALTER TABLE posts
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();

-- Function to update the "updated_at" column
CREATE OR REPLACE FUNCTION on_posts_update()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create the trigger
CREATE OR REPLACE TRIGGER update_posts_updated_at
    BEFORE UPDATE
    ON
        posts
    FOR EACH ROW
EXECUTE PROCEDURE on_posts_update();
