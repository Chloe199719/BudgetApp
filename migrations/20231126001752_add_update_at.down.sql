-- Add down migration script here
ALTER TABLE categories DROP COLUMN updated_at;

DROP TRIGGER update_categories_updated_at ON categories;
DROP FUNCTION update_updated_at_column();
