CREATE SCHEMA IF NOT EXISTS auth;

CREATE TABLE IF NOT EXISTS auth.credentials (
    id SERIAL PRIMARY KEY,
    name varchar(255) unique not null,
    hash varchar(255) unique not null,
    email varchar(255) unique not null,
    updated_at timestamp DEFAULT current_timestamp not null,
    created_at timestamp DEFAULT current_timestamp not null,
    deleted_at timestamp DEFAULT null
);


