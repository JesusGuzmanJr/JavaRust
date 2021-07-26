CREATE TYPE ACCOUNT_STATUS AS ENUM ('unverified', 'active', 'disabled', 'deleted');
CREATE TABLE IF NOT EXISTS account (
    id UUID PRIMARY KEY,
    created TIMESTAMP WITH TIME ZONE NOT NULL,
    updated TIMESTAMP WITH TIME ZONE NOT NULL,
    username TEXT NOT NULL UNIQUE,
    CHECK (octet_length(username) <= 16),
    CHECK (octet_length(username) >= 8),
    email TEXT NOT NULL UNIQUE,
    CHECK (octet_length(email) <= 64),
    password_hash TEXT NOT NULL,
    status ACCOUNT_STATUS NOT NULL
);