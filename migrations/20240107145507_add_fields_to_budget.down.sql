-- Add down migration script here
ALTER TABLE budgets DROP COLUMN IF EXISTS recurring;
ALTER TABLE budgets DROP COLUMN IF EXISTS duration_unix;