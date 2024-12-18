-- Add up migration script here
CREATE TABLE credentials
(
    id          UUID PRIMARY KEY,
    user_id     UUID        NOT NULL,
    private_key TEXT        NOT NULL,
    public_key  TEXT        NOT NULL,
    type        VARCHAR(50) NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW(),
    updated_at  TIMESTAMP DEFAULT NOW(),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX idx_id ON credentials (id);
CREATE INDEX idx_user_id ON credentials (user_id);
CREATE INDEX idx_type ON credentials (type);
CREATE INDEX idx_created_at ON credentials (created_at);
