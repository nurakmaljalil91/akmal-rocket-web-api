-- Your SQL goes here
CREATE TABLE todo
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    complete BOOLEAN NOT NULL DEFAULT 'f'
)