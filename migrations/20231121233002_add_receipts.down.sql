-- Add down migration script here
ALTER TABLE transactions DROP COLUMN IF EXISTS receipt_id;
DROP TABLE IF EXISTS receipts;
