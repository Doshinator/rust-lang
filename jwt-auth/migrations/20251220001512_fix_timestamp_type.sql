-- Add migration script here
-- Change TIMESTAMP to TIMESTAMPTZ
ALTER TABLE users 
ALTER COLUMN created_at TYPE TIMESTAMPTZ;