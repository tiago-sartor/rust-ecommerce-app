CREATE TABLE IF NOT EXISTS customers (
    id BIGSERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    phone TEXT CHECK (LENGTH(phone) > 9 AND LENGTH(phone) < 12),
    profile_image_url TEXT,
    cpf TEXT UNIQUE CHECK (LENGTH(cpf) = 11),
    company_name TEXT,
    cnpj TEXT UNIQUE CHECK (LENGTH(cnpj) = 14),
    state_registration TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    last_login TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    reset_token TEXT,
    reset_expires_at TIMESTAMP
);