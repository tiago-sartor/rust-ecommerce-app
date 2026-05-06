CREATE TABLE IF NOT EXISTS attribute_options (
    id BIGSERIAL PRIMARY KEY,
    attribute_id BIGINT NOT NULL REFERENCES attributes(id) ON DELETE CASCADE,
    group_id BIGINT REFERENCES attribute_groups(id) ON DELETE SET NULL,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    image TEXT,
    sort_order INT DEFAULT 0,
    created_at TIMESTAMPZ DEFAULT now(),
    updated_at TIMESTAMPZ DEFAULT now()
);