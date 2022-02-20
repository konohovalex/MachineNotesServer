CREATE TABLE IF NOT EXISTS user_account (
    user_id TEXT NOT NULL PRIMARY KEY,
    user_name VARCHAR(32) UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    password_hash TEXT,
    password_hash_salt TEXT,
    password_hash_algorithm TEXT,
    access_token TEXT NOT NULL UNIQUE,
    refresh_token TEXT NOT NULL UNIQUE
);