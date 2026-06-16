CREATE TABLE IF NOT EXISTS categories (
    id          BIGSERIAL   PRIMARY KEY,
    parent_id   BIGINT      REFERENCES categories(id) ON DELETE SET NULL,
    slug        TEXT        NOT NULL UNIQUE,
    name        TEXT        NOT NULL,
    description TEXT,
    image_url   TEXT,
    created_at  TIMESTAMPTZ DEFAULT now(),
    updated_at  TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id);