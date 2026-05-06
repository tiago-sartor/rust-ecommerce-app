CREATE TABLE IF NOT EXISTS attributes (
    id BIGSERIAL PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    image TEXT,
    created_at TIMESTAMPZ DEFAULT now(),
    updated_at TIMESTAMPZ DEFAULT now()
);