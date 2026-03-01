-- Add migration script here
-- workspaces for user
CREATE TABLE if NOT EXISTS workspaces (
    id bigserial PRIMARY KEY,
    name varchar(32) NOT NULL UNIQUE,
    owner_id bigint NOT NULL REFERENCES users(id),
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- alter users table to add ws_id
-- 修改 `users` 表
-- 新增一列 `ws_id`
-- 类型是 `bigint`（用于存 workspace 的 id）
-- `REFERENCES workspaces(id)` 是**外键约束**，表示这个值必须是 `workspaces` 表中存在的 id
ALTER TABLE users
    ADD COLUMN ws_id bigint REFERENCES workspaces(id);

ALTER TABLE chats
    ADD COLUMN ws_id bigint REFERENCES workspaces(id);

-- add super user 0 and workspace 0
BEGIN;
INSERT INTO users(id, fullname, email, password_hash)
 VALUES(0, 'super super', 'super@super.com', '');
INSERT INTO workspaces(id, name, owner_id)
 VALUES(0, 'none', 0);
UPDATE
    users
SET
    ws_id = 0
WHERE
    id = 0;
COMMIT;

-- alter user table to make ws_id not null
ALTER TABLE users
  ALTER COLUMN ws_id SET NOT NULL;
