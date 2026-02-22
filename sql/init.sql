CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY,
    fullname varchar(64) NOT NULL,
    email varchar(255) NOT NULL,

    -- hashed argon2 password
    password varchar(64) NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
);

-- create index for users for email
CREATE UNIQUE INDEX IF NOT EXISTS email_index ON users(email);

-- create chat type: single group, private_channel, public_channel
CREATE TYPE chat_type AS ENUM(
    'single',
    'group',
    'private_channel',
    'public_channel',
);

CREATE TABLE IF NOT EXISTS chats (
    id bigserial PRIMARY KEY,
    name varchar(128) NOT NULL UNIQUE,
    type chat_type NOT NULL,
    -- user id list
    number bigint[] NOT NULL,
    create_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
)

-- create message table
CREATE TABLE IF NOT EXISTS messages (
    id bigserial PRIMARY KEY,
    chat_id bigint NOT NULL,
    sender_id bigint NOT NULL,
    content text NOT NULL,
    images text[],
    create_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (chat_id) REFERENCES chats(id),
    FOREIGN KEY (sender_id) REFERENCES users(id),
)

CREATE INDEX IF NOT EXISTS chat_id_created_at_index ON messages(chat_id, create_at DESC);

CREATE INDEX IF NOT EXISTS serder_id_index ON message(sender_id);
