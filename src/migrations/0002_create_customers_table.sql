CREATE TABLE IF NOT EXISTS customers (
    id BIGSERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    phone TEXT NOT NULL CHECK (LENGTH(phone) > 9 AND LENGTH(phone) < 12),
    profile_image_url TEXT,
    cpf TEXT UNIQUE CHECK (LENGTH(cpf) = 11),
    company_name TEXT,
    cnpj TEXT UNIQUE CHECK (LENGTH(cnpj) = 14),
    state_registration TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_login TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    reset_token TEXT,
    reset_expires_at TIMESTAMPTZ
);