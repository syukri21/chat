-- Add up migration script here
CREATE TABLE users
(
    id         UUID PRIMARY KEY,
    username   VARCHAR(255) NOT NULL,
    email      VARCHAR(255) NOT NULL UNIQUE,
    password   TEXT         NOT NULL,
    pub_key    TEXT         NOT NULL,
    is_active  BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);


CREATE INDEX idx_users_username_is_active
    ON users (username, is_active);

CREATE INDEX idx_users_email_is_active
    ON users (email, is_active);

CREATE INDEX idx_users_username_email_is_active
    ON users (username, email, is_active);

CREATE INDEX idx_users_username
    ON users (username);

CREATE INDEX idx_users_email
    ON users (email);

CREATE INDEX idx_users_is_active
    ON users (is_active);