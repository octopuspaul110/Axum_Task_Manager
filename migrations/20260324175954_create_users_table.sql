-- Add migration script here

-- install uuid for postgres
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- create user table with the required columns
CREATE TABLE users (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email               TEXT UNIQUE NOT NULL,
    name                Text NOT NULL,
    password_hash       TEXT NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- create index
CREATE INDEX idx_users_email ON users(email);


