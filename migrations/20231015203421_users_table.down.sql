-- Add down migration script here
DROP TABLE IF EXISTS user_profile CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP DOMAIN IF EXISTS phone;