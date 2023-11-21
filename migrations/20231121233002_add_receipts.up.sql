-- Add up migration script here
CREATE TABLE IF NOT EXISTS receipts (
    id SERIAL PRIMARY KEY,
    transaction_id INT NOT NULL,
    user_Id UUID NOT NULL,
    receipt_url VARCHAR(255) NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT transaction_id FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id)
);

ALTER TABLE transactions ADD COLUMN IF NOT EXISTS receipt_id INT;
ALTER TABLE transactions ADD CONSTRAINT receipt_id FOREIGN KEY (receipt_id) REFERENCES receipts(id);
