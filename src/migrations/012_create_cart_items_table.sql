CREATE TABLE IF NOT EXISTS cart_items (
    id            BIGSERIAL       PRIMARY KEY,
    cart_id       BIGINT          NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
    product_id    BIGINT          NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity      INT             NOT NULL,
    created_at    TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ     NOT NULL DEFAULT now()
);
