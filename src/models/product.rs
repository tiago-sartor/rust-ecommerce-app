pub struct Product {
    pub id: u32,
    pub sku: String,
    pub name: String,
    pub price: f64,
    pub sale_price: Option<f64>,
    pub quantity: u32,
    pub attributes: Vec<(String, String)>,
    pub description: String,
    pub categories: Vec<String>,
    pub main_category: String,
    pub images: Vec<String>,
    pub tags: Vec<String>,
    pub is_active: bool,
    pub is_featured: bool,
    pub total_sales: u32,
    pub created_at: String,
    pub updated_at: String,
}

pub fn get_all_products() -> Vec<Product> {
    // This function would typically fetch products from a database
    vec![]
}

pub fn get_product_by_id(id: u32) -> Option<Product> {
    // This function would typically fetch a product by its ID from a database
    None
}

pub fn get_product_by_sku(sku: &str) -> Option<Product> {
    // This function would typically fetch a product by its SKU from a database
    None
}

pub fn is_on_sale(product: &Product) -> bool {
    product.sale_price.is_some() && product.sale_price.unwrap() < product.price
}

pub fn create_product(product: Product) -> Product {
    // This function would typically save a new product to a database
    product
}

pub fn update_product(id: u32, updated_product: Product) -> Option<Product> {
    // This function would typically update an existing product in a database
    None
}

pub fn delete_product(id: u32) -> bool {
    // This function would typically delete a product from a database
    false
}
