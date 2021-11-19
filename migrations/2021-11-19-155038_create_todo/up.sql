-- Your SQL goes here
CREATE TABLE todo (
    id       SERIAL PRIMARY KEY,
    name     TEXT NOT NULL,
    complete BOOLEAN NOT NULL DEFAULT 'f'
)