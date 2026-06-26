CREATE TABLE IF NOT EXISTS order_items (
    id            BIGSERIAL       PRIMARY KEY,
    order_id      BIGINT          NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id    BIGINT          NULL REFERENCES products(id) ON DELETE SET NULL,
    sku           TEXT            NOT NULL,
    name          TEXT            NOT NULL,
    quantity      INT             NOT NULL,
    price         DECIMAL(14, 2)  NOT NULL CHECK (price >= 0),
    sale_price    DECIMAL(14, 2)  NULL CHECK (sale_price >= 0 AND sale_price < price),
    created_at    TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ     NOT NULL DEFAULT now()
);
