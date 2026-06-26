CREATE TABLE IF NOT EXISTS customers (
    id                  BIGSERIAL    PRIMARY KEY,
    first_name          TEXT         NOT NULL,
    last_name           TEXT         NOT NULL,
    email               TEXT         NOT NULL UNIQUE,
    password_hash       TEXT         NOT NULL CHECK (LENGTH(password_hash) > 0), -- Prevents storing an empty string
    phone               TEXT         NOT NULL CHECK (LENGTH(phone) > 9 AND LENGTH(phone) < 12),
    profile_image_url   TEXT         NULL,
    cpf                 TEXT         NULL UNIQUE CHECK (LENGTH(cpf) = 11),
    cnpj                TEXT         NULL UNIQUE CHECK (LENGTH(cnpj) = 14),
    company_name        TEXT         NULL,
    state_registration  TEXT         NULL,
    is_subscribed       BOOLEAN      NOT NULL DEFAULT TRUE,
    last_active_at      TIMESTAMPTZ  NULL,
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT now(),
    reset_token         TEXT         NULL,
    reset_expires_at    TIMESTAMPTZ  NULL
);

CREATE INDEX IF NOT EXISTS idx_customers_reset_token ON customers(reset_token) WHERE reset_token IS NOT NULL;
