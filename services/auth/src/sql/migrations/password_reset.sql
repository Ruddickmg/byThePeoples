CREATE TABLE IF NOT EXISTS auth.password_reset (
  id char(32) PRIMARY KEY UNIQUE NOT NULL,
  user_id int NOT NULL REFERENCES auth.credentials(id) ON UPDATE CASCADE ON DELETE CASCADE,
  reset_token char(118) UNIQUE NOT NULL,
  name VARCHAR(255) NOT NULL,
  email citext UNIQUE NOT NULL,
  created_at timestamp DEFAULT current_timestamp not null
)
