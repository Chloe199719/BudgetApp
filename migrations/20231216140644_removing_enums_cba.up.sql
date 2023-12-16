-- Add up migration script here
ALTER TABLE transactions DROP COLUMN transaction_type;
ALTER TABLE transactions ADD COLUMN transaction_type VARCHAR(255) NOT NULL;
ALTER TABLE transactions DROP COLUMN currency;
ALTER TABLE transactions ADD COLUMN currency VARCHAR(20) NOT NULL;