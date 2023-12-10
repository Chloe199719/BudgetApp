-- Add up migration script here
ALTER TABLE transactions ADD COLUMN updated_at timestamptz NOT NULL DEFAULT NOW();

CREATE TRIGGER update_transactions_updated_at
BEFORE UPDATE ON transactions
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
