CREATE TABLE IF NOT EXISTS auth.failed_login (
    userId int PRIMARY KEY NOT NULL REFERENCES auth.credentials(id) ON UPDATE CASCADE ON DELETE CASCADE,
    updated_at timestamp DEFAULT current_timestamp not null,
    created_at timestamp DEFAULT current_timestamp not null,
    attempts smallint DEFAULT 1
);
