CREATE TABLE IF NOT EXISTS carts (
    id            BIGSERIAL    PRIMARY KEY,
    customer_id   BIGINT       NULL REFERENCES customers(id) ON DELETE CASCADE,
    session_id    TEXT         NULL REFERENCES tower_sessions.session(id) ON DELETE SET NULL,
    status        TEXT         NOT NULL,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);
