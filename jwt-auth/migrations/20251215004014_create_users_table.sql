-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Add user_id to game_accounts
ALTER TABLE game_accounts
ADD COLUMN user_id UUID REFERENCES users(id) ON DELETE CASCADE;

-- Create index for faster lookup
CREATE INDEX idx_game_accounts_user_id ON game_accounts(user_id);