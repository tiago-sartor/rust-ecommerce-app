CREATE TABLE IF NOT EXISTS admins (
    id                BIGSERIAL   PRIMARY KEY,
    first_name        TEXT         NOT NULL,
    last_name         TEXT         NOT NULL,
    email             TEXT         NOT NULL UNIQUE,
    password_hash     TEXT         NOT NULL,
    phone             TEXT         NOT NULL CHECK (LENGTH(phone) = 11),
    profile_image_url TEXT,
    role              TEXT         NOT NULL DEFAULT 'admin',
    is_active         BOOLEAN      NOT NULL DEFAULT TRUE,
    last_login        TIMESTAMPTZ,
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ  NOT NULL DEFAULT now(),
    reset_token       TEXT,
    reset_expires_at  TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_admins_email ON admins(email);
CREATE INDEX IF NOT EXISTS idx_admins_reset_token ON admins(reset_token);
