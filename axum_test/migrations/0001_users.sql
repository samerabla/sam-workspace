--drop TABLE users;

CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password Text NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE pending_users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    verification_token TEXT NOT NULL
);

CREATE TABLE password_reset_tokens (
    id Serial Not Null UNIQUE,
    email TEXT NOT NULL,
    token TEXT NOT NULL
);

create table errors (
    id Serial Not Null UNIQUE,
    module VARCHAR(255),
    file VARCHAR(255),
    line INT,
    message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)