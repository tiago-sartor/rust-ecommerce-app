CREATE TABLE IF NOT EXISTS attribute_options (
    id            BIGSERIAL    PRIMARY KEY,
    attribute_id  BIGINT       NOT NULL REFERENCES attributes(id) ON DELETE CASCADE,
    group_id      BIGINT       NULL REFERENCES attribute_groups(id) ON DELETE SET NULL,
    slug          TEXT         NOT NULL UNIQUE,
    name          TEXT         NOT NULL,
    description   TEXT         NULL,
    image_url     TEXT         NULL,
    sort_order    INT          NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_attribute_options_attribute_id ON attribute_options(attribute_id);
CREATE INDEX IF NOT EXISTS idx_attribute_options_group_id ON attribute_options(group_id);
