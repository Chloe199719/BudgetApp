-- Add down migration script here
-- Drop tables and types in reverse order of creation, considering dependencies
ALTER TABLE "transactions" DROP CONSTRAINT IF EXISTS CATEGORY_ID;
ALTER TABLE "budgets" DROP CONSTRAINT IF EXISTS CATEGORY_ID;
ALTER TABLE "transactions" DROP CONSTRAINT IF EXISTS transaction_type;
ALTER TABLE "alerts" DROP CONSTRAINT IF EXISTS alert_type;

DROP TABLE IF EXISTS "alerts";
DROP TABLE IF EXISTS "transactions";
DROP TABLE IF EXISTS "budgets";

-- Assuming dependencies on 'categories' have been resolved
DROP TABLE IF EXISTS "categories";

-- Drop types, assuming no dependencies
DROP TYPE IF EXISTS transaction_type;
DROP TYPE IF EXISTS alert_type;
