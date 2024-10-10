CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  firstname VARCHAR(255) NOT NULL,
  lastname VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT true
);

-- ALTER TABLE authors ADD CONSTRAINT unique_email UNIQUE (email);