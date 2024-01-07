-- Add down migration script here
DROP TRIGGER IF EXISTS update_budgets_updated_at ON budgets;
ALTER TABLE budgets DROP COLUMN IF EXISTS updated_at;