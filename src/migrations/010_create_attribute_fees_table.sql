CREATE TABLE IF NOT EXISTS attribute_fees (
    id            BIGSERIAL       PRIMARY KEY,
    product_id    BIGINT          NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    attribute_id  BIGINT          NOT NULL REFERENCES attributes(id) ON DELETE CASCADE,
    option_id     BIGINT          NOT NULL REFERENCES attribute_options(id) ON DELETE CASCADE,
    fee_type      TEXT            NOT NULL,
    fee           DECIMAL(14, 2)  NULL,
    created_at    TIMESTAMPTZ     NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ     NOT NULL DEFAULT now(),

    CONSTRAINT unique_product_attribute_option UNIQUE (product_id, attribute_id, option_id)
);
