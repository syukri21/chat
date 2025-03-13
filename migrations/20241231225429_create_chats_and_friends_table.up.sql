CREATE TABLE chats
(
    id         UUID PRIMARY KEY,
    name       VARCHAR(255) NOT NULL,
    is_group   BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE chat_members
(
    id        UUID PRIMARY KEY,
    chat_id   UUID      NOT NULL REFERENCES chats (id),
    user_id   UUID      NOT NULL REFERENCES users (id),
    joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE messages
(
    id           UUID PRIMARY KEY,
    chat_id      UUID        NOT NULL REFERENCES chats (id),
    sender_id    UUID        NOT NULL REFERENCES users (id),
    content      TEXT        NOT NULL,
    message_type VARCHAR(50) NOT NULL,
    message_key  VARCHAR(50) NOT NULL,
    sent_at      TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE message_read_receipts
(
    id         UUID PRIMARY KEY,
    message_id UUID      NOT NULL REFERENCES messages (id),
    user_id    UUID      NOT NULL REFERENCES users (id),
    read_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE message_reactions
(
    id         UUID PRIMARY KEY,
    message_id UUID        NOT NULL REFERENCES messages (id),
    user_id    UUID        NOT NULL REFERENCES users (id),
    reaction   VARCHAR(50) NOT NULL,
    reacted_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE attachments
(
    id          UUID PRIMARY KEY,
    message_id  UUID         NOT NULL REFERENCES messages (id),
    file_url    VARCHAR(255) NOT NULL,
    file_type   VARCHAR(50)  NOT NULL,
    file_size   INT          NOT NULL,
    uploaded_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP
);
