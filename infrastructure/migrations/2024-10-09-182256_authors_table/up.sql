CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  firstname VARCHAR(255) NOT NULL,
  lastname VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  is_active BOOLEAN NOT NULL DEFAULT true
);

ALTER TABLE authors
ADD CONSTRAINT unique_email UNIQUE (email);

INSERT INTO authors (firstname, lastname, email, is_active) VALUES
    ('Any', 'More', 'anymore@example.com', TRUE),
    ('Elek', 'Teszt', 'teszt@elek.hu', TRUE),
    ('Tina', 'Klemen', 'klemen@tina.hu', TRUE);