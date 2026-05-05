CREATE TABLE IF NOT EXISTS admins (
    id                BIGSERIAL  PRIMARY KEY,
    first_name        TEXT       NOT NULL,
    last_name         TEXT       NOT NULL,
    email             TEXT       NOT NULL UNIQUE,
    password_hash     TEXT       NOT NULL,
    phone             TEXT       NOT NULL CHECK (LENGTH(phone) = 11),
    profile_image_url TEXT,
    role              TEXT       DEFAULT 'admin',
    is_active         BOOLEAN    DEFAULT TRUE,
    last_login        TIMESTAMP,
    created_at        TIMESTAMP  DEFAULT CURRENT_TIMESTAMP,
    updated_at        TIMESTAMP  DEFAULT CURRENT_TIMESTAMP,
    reset_token       TEXT,
    reset_expires_at  TIMESTAMP
);