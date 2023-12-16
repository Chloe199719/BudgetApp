-- Add up migration script here
ALTER TABLE transactions DROP COLUMN currency;
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS currency currencys_type NOT NULL DEFAULT 'EUR';
