CREATE TABLE IF NOT EXISTS attribute_groups (
    id            BIGSERIAL    PRIMARY KEY,
    attribute_id  BIGINT       NOT NULL REFERENCES attributes(id) ON DELETE CASCADE,
    slug          TEXT         NOT NULL UNIQUE,
    name          TEXT         NOT NULL,
    description   TEXT         NULL,
    image_url     TEXT         NULL,
    sort_order    INT          NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_attribute_groups_attribute_id ON attribute_groups(attribute_id);