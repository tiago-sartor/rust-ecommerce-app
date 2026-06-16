CREATE TABLE IF NOT EXISTS customers (
    id                 BIGSERIAL    PRIMARY KEY,
    first_name         TEXT         NOT NULL,
    last_name          TEXT         NOT NULL,
    email              TEXT         NOT NULL UNIQUE,
    password_hash      TEXT         NOT NULL,
    phone              TEXT         NOT NULL CHECK (LENGTH(phone) > 9 AND LENGTH(phone) < 12),
    profile_image_url  TEXT,
    cpf                TEXT         UNIQUE CHECK (LENGTH(cpf) = 11),
    cnpj               TEXT         UNIQUE CHECK (LENGTH(cnpj) = 14),
    company_name       TEXT,
    state_registration TEXT,
    is_subscribed      BOOLEAN      NOT NULL DEFAULT TRUE,
    last_login         TIMESTAMPTZ,
    created_at         TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ  NOT NULL DEFAULT now(),
    reset_token        TEXT,
    reset_expires_at   TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_customers_reset_token ON customers(reset_token) WHERE reset_token IS NOT NULL;
