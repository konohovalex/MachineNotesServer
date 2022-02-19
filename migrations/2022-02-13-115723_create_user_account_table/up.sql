CREATE TABLE IF NOT EXISTS user_account (
    user_id TEXT NOT NULL PRIMARY KEY,
    user_name VARCHAR(32),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    password_hash TEXT,
    password_hash_salt TEXT,
    password_hash_algorithm TEXT,
    UNIQUE(user_name)
);

CREATE TABLE IF NOT EXISTS auth_token (
    user_id TEXT NOT NULL REFERENCES user_account(user_id) ON DELETE CASCADE,
    token TEXT NOT NULL PRIMARY KEY,
    UNIQUE(user_id)
);