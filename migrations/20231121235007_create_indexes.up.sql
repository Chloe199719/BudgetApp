-- Add up migration script here
CREATE INDEX IF NOT EXISTS idx_transactions_user_id_FK ON transactions (user_id);
CREATE INDEX IF NOT EXISTS idx_transactions_category_id_FK ON transactions (category_id);
CREATE INDEX IF NOT EXISTS idx_budgets_user_id_FK ON budgets (user_id);
CREATE INDEX IF NOT EXISTS idx_budgets_category_id_FK ON budgets (category_id);
CREATE INDEX IF NOT EXISTS idx_alerts_user_id_FK ON alerts (user_id);
CREATE INDEX IF NOT EXISTS idx_receipts_user_id_FK ON receipts (user_id);
CREATE INDEX IF NOT EXISTS idx_receipts_transaction_id_FK ON receipts (transaction_id);
CREATE INDEX If NOT EXISTS idx_categories_user_id_FK ON categories (user_id);
CREATE INDEX IF NOT EXISTS idx_transactions_receipt_id_FK ON transactions (receipt_id);