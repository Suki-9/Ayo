CREATE TABLE IF NOT EXISTS "entity_group" (
  "id"              TEXT NOT NULL PRIMARY KEY,
  "inheritance_id"  TEXT,
  "name"            TEXT,
  "summary"         TEXT,
  "owner_id"        TEXT,
  "chmod"           BYTEA NOT NULL,
  "created_at"      BIGINT NOT NULL,
  "deleted_at"      BIGINT,
  "is_archived"     BOOLEAN NOT NULL DEFAULT FALSE,
  "is_deleted"      BOOLEAN NOT NULL DEFAULT FALSE
);
