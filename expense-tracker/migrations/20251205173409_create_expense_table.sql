-- Add migration script here
CREATE TABLE IF NOT EXISTS expenses (
    id UUID PRIMARY KEY NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    category TEXT NOT NULL,
    description TEXT NOT NULL,
    date DATE NOT NULL
);