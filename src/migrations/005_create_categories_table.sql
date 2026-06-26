CREATE TABLE IF NOT EXISTS categories (
    id           BIGSERIAL    PRIMARY KEY,
    parent_id    BIGINT       NULL REFERENCES categories(id) ON DELETE SET NULL,
    slug         TEXT         NOT NULL UNIQUE,
    name         TEXT         NOT NULL,
    description  TEXT         NULL, 
    image_url    TEXT         NULL, 
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id);