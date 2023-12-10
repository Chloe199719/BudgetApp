-- Add up migration script here

CREATE TYPE currencys_type AS ENUM ('EUR','USD','WON','YEN','POUND');

CREATE TABLE IF NOT EXISTS currencys (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    convert_to currencys_type NOT NULL,
    convert_from currencys_type NOT NULL,
    value FLOAT NOT NULL,
    generated_at timestamptz NOT NULL DEFAULT NOW()
);

ALTER TABLE user_profile ADD COLUMN IF NOT EXISTS currency currencys_type NOT NULL DEFAULT 'EUR';
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS currency currencys_type NOT NULL DEFAULT 'EUR';