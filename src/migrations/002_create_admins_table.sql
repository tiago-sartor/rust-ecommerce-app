CREATE TABLE IF NOT EXISTS admins (
    id                 BIGSERIAL    PRIMARY KEY,
    first_name         TEXT         NOT NULL,
    last_name          TEXT         NOT NULL,
    email              TEXT         NOT NULL UNIQUE,
    password_hash      TEXT         NOT NULL CHECK (LENGTH(password_hash) > 0), -- Prevents storing an empty string
    phone              TEXT         NOT NULL CHECK (LENGTH(phone) = 11),
    profile_image_url  TEXT         NULL,
    role               TEXT         NOT NULL,
    is_active          BOOLEAN      NOT NULL DEFAULT TRUE,
    last_active_at     TIMESTAMPTZ  NULL,
    created_at         TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ  NOT NULL DEFAULT now(),
    reset_token        TEXT         NULL,
    reset_expires_at   TIMESTAMPTZ  NULL
);

CREATE INDEX IF NOT EXISTS idx_admins_role ON admins(role);
CREATE INDEX IF NOT EXISTS idx_admins_reset_token ON admins(reset_token) WHERE reset_token IS NOT NULL;

-- Create a mock admin user for first time access. 
-- Password: admin123
INSERT INTO admins (first_name, last_name, email, password_hash, phone, role)
VALUES (
    'Admin',
    'User',
    'admin@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$TkXkr3krsC+wDRCrVuc2KQ$RrThuGPwiGhcfpBUwjvLc7e6EkNfpAL+M18Ek6iKTQk',
    '11999999999',
    'admin'
) ON CONFLICT (email) DO NOTHING;
