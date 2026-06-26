CREATE TABLE IF NOT EXISTS attributes (
    id          BIGSERIAL   PRIMARY KEY,
    slug        TEXT        NOT NULL UNIQUE,
    name        TEXT        NOT NULL,
    description TEXT        NULL,
    image_url   TEXT        NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
