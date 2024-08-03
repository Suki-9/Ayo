CREATE TABLE IF NOT EXISTS "users" (
  "id"          TEXT PRIMARY KEY,
  "name"        TEXT NOT NULL,
  "summary"     TEXT,
  "role"        TEXT NOT NULL,
  "password"    TEXT NOT NULL,
  "root"        TEXT NOT NULL,
  "created_at"  BIGINT NOT NULL,
  "deleted_at"  BIGINT,
  "is_deleted"  BOOLEAN NOT NULL DEFAULT FALSE
);
