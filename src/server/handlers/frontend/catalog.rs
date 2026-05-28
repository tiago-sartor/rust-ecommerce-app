use axum::{
    Json,
    extract::{Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub id: i64,
    pub slug: String,
    pub name: String,
    pub price: f64,
    pub sale_price: Option<f64>,
    pub description: String,
    pub image: String,
    pub category: String,
}

pub fn get_mock_products() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            slug: "laptop-pro".to_string(),
            name: "Laptop Pro".to_string(),
            price: 1299.99,
            sale_price: Some(999.99),
            description: "High-performance laptop for professionals".to_string(),
            image: "/images/laptop-pro.jpg".to_string(),
            category: "Electronics".to_string(),
        },
        Product {
            id: 2,
            slug: "wireless-mouse".to_string(),
            name: "Wireless Mouse".to_string(),
            price: 49.99,
            sale_price: None,
            description: "Ergonomic wireless mouse with long battery life".to_string(),
            image: "/images/wireless-mouse.jpg".to_string(),
            category: "Accessories".to_string(),
        },
        Product {
            id: 3,
            slug: "usb-c-hub".to_string(),
            name: "USB-C Hub".to_string(),
            price: 79.99,
            sale_price: Some(59.99),
            description: "7-in-1 USB-C hub with multiple ports".to_string(),
            image: "/images/usb-c-hub.jpg".to_string(),
            category: "Accessories".to_string(),
        },
        Product {
            id: 4,
            slug: "mechanical-keyboard".to_string(),
            name: "Mechanical Keyboard".to_string(),
            price: 149.99,
            sale_price: None,
            description: "Premium mechanical keyboard with RGB lighting".to_string(),
            image: "/images/mechanical-keyboard.jpg".to_string(),
            category: "Accessories".to_string(),
        },
        Product {
            id: 5,
            slug: "4k-monitor".to_string(),
            name: "4K Monitor".to_string(),
            price: 599.99,
            sale_price: Some(499.99),
            description: "32-inch 4K UHD monitor with HDR support".to_string(),
            image: "/images/4k-monitor.jpg".to_string(),
            category: "Electronics".to_string(),
        },
    ]
}

pub fn get_mock_product_by_slug(slug: &str) -> Option<Product> {
    get_mock_products().into_iter().find(|p| p.slug == slug)
}

/// GET / - Home page
pub async fn home_page() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Welcome - E-Commerce Store</title>
    <link rel="stylesheet" href="/style.css">
</head>
<body>
    <div class="container">
        <h1>Welcome to Our E-Commerce Store</h1>
        <p>Find the best products at unbeatable prices.</p>
        <nav>
            <a href="/products">Browse Products</a>
            <a href="/cart">View Cart</a>
        </nav>
        <section class="featured-products">
            <h2>Featured Products</h2>
            <div class="products-grid">
                <div class="product-card">
                    <h3>Laptop Pro</h3>
                    <p class="price"><s>$1299.99</s> <span class="sale">$999.99</span></p>
                    <a href="/products/1">View Details</a>
                </div>
                <div class="product-card">
                    <h3>4K Monitor</h3>
                    <p class="price"><s>$599.99</s> <span class="sale">$499.99</span></p>
                    <a href="/products/5">View Details</a>
                </div>
                <div class="product-card">
                    <h3>USB-C Hub</h3>
                    <p class="price"><s>$79.99</s> <span class="sale">$59.99</span></p>
                    <a href="/products/3">View Details</a>
                </div>
            </div>
        </section>
    </div>
</body>
</html>"#,
    )
}

/// GET /products - Products listing page
pub async fn catalog_page() -> Json<Vec<Product>> {
    Json(get_mock_products())
}

/// GET /products/:slug - Product detail page
pub async fn product_detail_page(Path(slug): Path<String>) -> Result<Json<Product>, StatusCode> {
    get_mock_product_by_slug(&slug).map(Json).ok_or(StatusCode::NOT_FOUND)
}
