CREATE TABLE IF NOT EXISTS attribute_options (
    id           BIGSERIAL    PRIMARY KEY,
    attribute_id BIGINT       NOT NULL REFERENCES attributes(id) ON DELETE CASCADE,
    group_id     BIGINT       REFERENCES attribute_groups(id) ON DELETE SET NULL,
    slug         TEXT         NOT NULL UNIQUE,
    name         TEXT         NOT NULL,
    description  TEXT,
    image_url    TEXT,
    sort_order   INT          DEFAULT 0,
    created_at   TIMESTAMPTZ  DEFAULT now(),
    updated_at   TIMESTAMPTZ  DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_attribute_options_attribute_id ON attribute_options(attribute_id);
CREATE INDEX IF NOT EXISTS idx_attribute_options_group_id     ON attribute_options(group_id);
