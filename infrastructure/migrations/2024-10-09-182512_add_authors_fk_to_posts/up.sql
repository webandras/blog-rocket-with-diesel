ALTER TABLE posts
ADD COLUMN author_id INT,
ADD CONSTRAINT fk_author_id
    FOREIGN KEY(author_id)
    REFERENCES authors(id)
    ON DELETE SET NULL
    ;