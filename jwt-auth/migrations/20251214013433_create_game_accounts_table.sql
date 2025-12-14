-- Add migration script here
CREATE TABLE IF NOT EXISTS game_accounts (
    id UUID PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    platform TEXT NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    total_hours_played INTEGER NOT NULL DEFAULT 0
);