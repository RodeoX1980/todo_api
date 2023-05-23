-- Add migration script here
DROP TABLE IF EXISTS task;
CREATE TABLE task
(
    id         TEXT      NOT NULL PRIMARY KEY,
    body       TEXT,
    status     VARCHAR(2),
    created_at timestamp NOT NULL default current_timestamp,
    updated_at timestamp NOT NULL default current_timestamp
)
