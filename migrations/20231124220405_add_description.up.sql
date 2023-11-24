-- Add up migration script here
ALTER TABLE categories ADD COLUMN description TEXT NOT NULL;
