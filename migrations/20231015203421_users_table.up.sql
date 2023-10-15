-- Add up migration script here

CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    display_name TEXT NOT NULL,
    unique_name TEXT NOT NULL UNIQUE,
    is_active BOOLEAN  DEFAULT FALSE,
    is_staff BOOLEAN  DEFAULT FALSE,
    is_superuser BOOLEAN  DEFAULT FALSE,
    thumbnail TEXT NULL,
    data_joined TIMESTAMPTZ NOT NULL DEFAULT NOW()
)

CREATE INDEX IF NOT EXISTS users_id_email_is_active_indx ON users (id, email, unique_name, is_active);

CREATE DOMAIN phone AS TEXT CHECK(
    octet_length(VALUE) BETWEEN 1
    /*+*/
    + 8 AND 1
    /*+*/
    + 15 + 3
    AND VALUE ~ '^\+\d+$'
);

CREATE TABLE IF NOT EXISTS user_profile (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE,
    phone_number phone NULL,
    birth_date DATE NULL,
    github_link TEXT NULL,
    twitter_link TEXT NULL,
    facebook_link TEXT NULL,
    linkedin_link TEXT NULL,
    twitch_link TEXT NULL,
    youtube_link TEXT NULL,
    pronouns TEXT NULL,
    about_me TEXT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
)

CREATE INDEX IF NOT EXISTS users_detail_id_user_id ON user_profile (id, user_id);