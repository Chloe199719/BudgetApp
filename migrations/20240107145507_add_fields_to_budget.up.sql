-- Add up migration script here
ALTER TABLE budgets ADD COLUMN IF NOT EXISTS recurring boolean NOT NULL DEFAULT false;
ALTER TABLE budgets ADD COLUMN IF NOT EXISTS duration_unix int NOT NULL 