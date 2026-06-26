CREATE TABLE IF NOT EXISTS addresses (
    id            BIGSERIAL    PRIMARY KEY,
    customer_id   BIGINT       NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    street        TEXT         NOT NULL,
    number        INT          NULL,
    complement    TEXT         NOT NULL,
    neighborhood  TEXT         NOT NULL,
    city          TEXT         NOT NULL,
    state         TEXT         NOT NULL,
    postcode      TEXT         NOT NULL CHECK (LENGTH(postcode) = 8),
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_addresses_customer_id ON addresses(customer_id);