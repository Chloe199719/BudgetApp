-- Add down migration script here
ALTER TABLE transactions DROP COLUMN updated_at;
DROP TRIGGER update_transactions_updated_at ON transactions;