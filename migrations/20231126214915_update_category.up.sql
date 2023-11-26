-- Add up migration script here
ALTER TABLE categories ADD COLUMN is_default BOOLEAN NOT NULL DEFAULT FALSE;
