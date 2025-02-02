-- Add up migration script here
CREATE TABLE sessions
(
    id         UUID PRIMARY KEY,
    session_id UUID UNIQUE NOT NULL, -- this should be jti, and unique
    user_id    UUID,
    user_agent VARCHAR(255),
    ip_address VARCHAR(20),
    created_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX idx_sessions_session_id ON sessions (session_id);
CREATE INDEX idx_sessions_user_id ON sessions (user_id);
