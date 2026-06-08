-- V2 creates the users table for auth and persistence between sessions.

-- Stores verified users.
CREATE TABLE IF NOT EXISTS ed_api.users (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id text NOT NULL,
  username text NOT NULL,
  email text NOT NULL,
  created_at timestamptz(3) NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS users_cat_idx
  ON ed_api.users (created_at DESC);

CREATE TABLE IF NOT EXISTS ed_api.sessions (
  id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id uuid NOT NULL REFERENCES ed_api.users(id),
  created_at timestamptz(3) NOT NULL,
  expires_at timestamptz(3) NOT NULL
);

CREATE INDEX IF NOT EXISTS sessions_uid_idx
  ON ed_api.sessions (user_id, created_at DESC);
