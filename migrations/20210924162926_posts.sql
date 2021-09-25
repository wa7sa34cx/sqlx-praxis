CREATE TABLE IF NOT EXISTS posts
(
    id        INTEGER PRIMARY KEY NOT NULL,
    title     TEXT                NOT NULL,
    date      TEXT                NOT NULL,
    published BOOLEAN             NOT NULL DEFAULT 0
);