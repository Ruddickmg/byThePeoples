CREATE SCHEMA IF NOT EXISTS auth;

CREATE TABLE IF NOT EXISTS auth.credentials (
    id SERIAL PRIMARY KEY,
    name varchar(255) not null,
    hash varchar(255) not null,
    email varchar(255) not null,
    updated_at timestamp DEFAULT current_timestamp not null,
    created_at timestamp DEFAULT current_timestamp not null
);


