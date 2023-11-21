-- Add down migration script here
DROP INDEX IF EXISTS idx_transactions_user_id_FK;
DROP INDEX IF EXISTS idx_transactions_category_id_FK;
DROP INDEX IF EXISTS idx_budgets_user_id_FK;
DROP INDEX IF EXISTS idx_budgets_category_id_FK;
DROP INDEX IF EXISTS idx_alerts_user_id_FK;
DROP INDEX IF EXISTS idx_receipts_user_id_FK;
DROP INDEX IF EXISTS idx_receipts_transaction_id_FK;
DROP INDEX IF EXISTS idx_categories_user_id_FK;
DROP INDEX IF EXISTS idx_transactions_receipt_id_FK;
