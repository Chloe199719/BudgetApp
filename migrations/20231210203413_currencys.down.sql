-- Add down migration script here
ALTER TABLE user_profile DROP COLUMN IF EXISTS currency;
ALTER TABLE transactions DROP COLUMN IF EXISTS currency;
DROP TABLE IF EXISTS currencys;
DROP TYPE IF EXISTS currencys_type;