CREATE TABLE IF NOT EXISTS products (
    id                BIGSERIAL       PRIMARY KEY,
    sku               TEXT            NOT NULL UNIQUE,
    name              TEXT            NOT NULL,
    price             DECIMAL(14, 2)  NOT NULL CHECK (price >= 0),
    sale_price        DECIMAL(14, 2)  CHECK (sale_price >= 0 AND sale_price < price),
    description       TEXT,
    main_category_id  BIGINT          REFERENCES categories(id) ON DELETE SET NULL,
    is_active         BOOLEAN         DEFAULT TRUE,
    is_featured       BOOLEAN         DEFAULT FALSE,
    total_sales       BIGINT          DEFAULT 0,
    created_at        TIMESTAMPTZ     DEFAULT now(),
    updated_at        TIMESTAMPTZ     DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_products_price             ON  products(price);
CREATE INDEX IF NOT EXISTS idx_products_sale_price        ON  products(sale_price) WHERE sale_price IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_products_main_category_id  ON  products(main_category_id);
CREATE INDEX IF NOT EXISTS idx_products_is_featured       ON  products(is_featured) WHERE is_featured = TRUE;
CREATE INDEX IF NOT EXISTS idx_products_total_sales       ON  products(total_sales);
CREATE INDEX IF NOT EXISTS idx_products_created_at        ON  products(created_at);
