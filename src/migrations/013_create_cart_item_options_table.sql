CREATE TABLE IF NOT EXISTS cart_item_options (
    id            BIGSERIAL  PRIMARY KEY,
    cart_item_id  BIGINT     NOT NULL REFERENCES cart_items(id) ON DELETE CASCADE,
    attribute_id  BIGINT     NOT NULL REFERENCES attributes(id) ON DELETE CASCADE,
    option_id     BIGINT     NOT NULL REFERENCES attribute_options(id) ON DELETE CASCADE,
    
    -- Ensures a customer can't pick two different options for the exact same product attribute
    CONSTRAINT unique_cart_item_attribute UNIQUE (cart_item_id, attribute_id) 
);
