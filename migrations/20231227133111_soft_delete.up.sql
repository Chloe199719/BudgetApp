-- Add up migration script here
ALTER TABLE transactions ADD COLUMN deleted BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE categories ADD COLUMN deleted BOOLEAN NOT NULL DEFAULT FALSE;
