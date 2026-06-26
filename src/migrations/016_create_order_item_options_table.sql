CREATE TABLE IF NOT EXISTS order_item_options (
    id              BIGSERIAL       PRIMARY KEY,
    order_item_id   BIGINT          NOT NULL REFERENCES order_items(id) ON DELETE CASCADE,
    attribute_id    BIGINT          NULL REFERENCES attributes(id) ON DELETE SET NULL,
    attribute_name  TEXT            NOT NULL,
    option_id       BIGINT          NULL REFERENCES attribute_options(id) ON DELETE SET NULL,
    option_name     TEXT            NOT NULL,
    option_fee      DECIMAL(14, 2)  NULL,
    
    -- Ensures a customer can't pick two different options for the exact same product attribute
    CONSTRAINT unique_order_item_attribute UNIQUE (order_item_id, attribute_id) 
);
