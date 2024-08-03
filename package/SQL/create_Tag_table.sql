CREATE TABLE IF NOT EXISTS "tag" (
  "id"          TEXT NOT NULL PRIMARY KEY,
  "value"       TEXT,
  "owner_id"    TEXT,
  "bind_type"   TEXT NOT NULL,
  "parent_id"   TEXT NOT NULL,
  "child_id"    TEXT NOT NULL,
  "created_at"  BIGINT NOT NULL
);
