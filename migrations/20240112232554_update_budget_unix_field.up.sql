-- Add up migration script here
ALTER TABLE budgets ALTER COLUMN duration_unix TYPE bigint;
