-- Add up migration script here
ALTER TABLE transactions DROP COLUMN type;
ALTER TABLE transactions ADD COLUMN transaction_type transaction_type NOT NULL;
