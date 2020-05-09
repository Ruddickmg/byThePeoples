CREATE TABLE IF NOT EXISTS password_reset (
  user_id int PRIMARY KEY NOT NULL REFERENCES auth.credentials(id) ON UPDATE CASCADE ON DELETE CASCADE,
  reset_id char(32) UNIQUE NOT NULL,
  created_at timestamp DEFAULT current_timestamp not null
)