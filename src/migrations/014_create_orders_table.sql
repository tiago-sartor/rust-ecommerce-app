CREATE TABLE IF NOT EXISTS orders (
    id              BIGSERIAL       PRIMARY KEY,
    customer_id     BIGINT          NULL REFERENCES customers(id) ON DELETE SET NULL,
    status          TEXT            NOT NULL,
    discount        DECIMAL(14, 2)  NOT NULL CHECK (discount >= 0),
    shipping_total  DECIMAL(14, 2)  NOT NULL CHECK (shipping_total >= 0),
    total           DECIMAL(14, 2)  NOT NULL CHECK (total >= 0),
    coupons         TEXT            NULL,
    created_at      TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ     NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
