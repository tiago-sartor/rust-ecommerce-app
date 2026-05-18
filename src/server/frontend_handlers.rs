use crate::models::customer::Customer;
use crate::utils::errors::AppError;
use crate::utils::helpers;
use axum::{
    Json,
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn customer_account() -> Html<String> {
    Html("<h1>My Account</h1>".to_string())
}

pub async fn customer_edit_account() -> Html<String> {
    Html("<h1>Edit Account</h1>".to_string())
}

pub async fn customer_password() -> Html<String> {
    Html("<h1>Change Password</h1>".to_string())
}

pub async fn customer_orders() -> Html<String> {
    Html("<h1>My Orders</h1>".to_string())
}

pub async fn customer_order_details() -> Html<String> {
    Html("<h1>Order Details</h1>".to_string())
}

pub async fn customer_address() -> Html<String> {
    Html("<h1>My Addresses</h1>".to_string())
}

pub async fn customer_edit_address() -> Html<String> {
    Html("<h1>Edit Address</h1>".to_string())
}

pub async fn customer_wishlist() -> Html<String> {
    Html("<h1>My Wishlist</h1>".to_string())
}

pub async fn login_page() -> Html<String> {
    Html("<h1>Login Page</h1>".to_string())
}

pub async fn customer_login_post(State(pool): State<PgPool>, session: Session, Form(payload): Form<LoginPayload>) -> Result<impl IntoResponse, AppError> {
    match Customer::get_by_email(&pool, &payload.email).await? {
        Some(customer) if crate::utils::password::verify_password(&customer.password_hash, &payload.password) => {
            session.insert("customer_id", customer.id).await?;
            session.insert("csrf_token", helpers::generate_random_token(64)).await?;

            Ok(Redirect::to("/").into_response())
        }
        _ => Ok(Redirect::to("/login").into_response()),
    }
}

pub async fn customer_logout(session: Session) -> Result<impl IntoResponse, AppError> {
    session.clear().await;
    helpers::regenerate_session(&session).await?;

    Ok(Redirect::to("/login").into_response())
}

// Mock data structures
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CartItem {
    pub id: i64,
    pub product_id: i64,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cart {
    pub items: Vec<CartItem>,
    pub total: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckoutData {
    pub cart_total: f64,
    pub shipping_cost: f64,
    pub tax: f64,
    pub grand_total: f64,
}

// Mock database functions
fn get_mock_products() -> Vec<Product> {
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

fn get_mock_product_by_id(id: i64) -> Option<Product> {
    get_mock_products().into_iter().find(|p| p.id == id)
}

fn get_mock_product_by_slug(slug: &str) -> Option<Product> {
    get_mock_products().into_iter().find(|p| p.slug == slug)
}

fn get_mock_cart() -> Cart {
    Cart {
        items: vec![
            CartItem {
                id: 1,
                product_id: 1,
                name: "Laptop Pro".to_string(),
                price: 999.99,
                quantity: 1,
            },
            CartItem {
                id: 2,
                product_id: 3,
                name: "USB-C Hub".to_string(),
                price: 59.99,
                quantity: 2,
            },
        ],
        total: 1119.97,
    }
}

// Handler implementations

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

/// GET /cart - Cart page
pub async fn cart_page() -> Json<Cart> {
    Json(get_mock_cart())
}

/// GET /checkout - Checkout page
pub async fn checkout_page() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Checkout - E-Commerce Store</title>
    <link rel="stylesheet" href="/style.css">
</head>
<body>
    <div class="container">
        <h1>Checkout</h1>
        <section class="checkout-form">
            <div class="order-summary">
                <h2>Order Summary</h2>
                <div class="summary-item">
                    <span>Subtotal:</span>
                    <span>$1,119.97</span>
                </div>
                <div class="summary-item">
                    <span>Shipping:</span>
                    <span>$15.00</span>
                </div>
                <div class="summary-item">
                    <span>Tax:</span>
                    <span>$89.60</span>
                </div>
                <div class="summary-item total">
                    <span>Total:</span>
                    <span>$1,224.57</span>
                </div>
            </div>
            <form method="POST" action="/checkout">
                <fieldset>
                    <legend>Shipping Information</legend>
                    <input type="text" name="first_name" placeholder="First Name" required>
                    <input type="text" name="last_name" placeholder="Last Name" required>
                    <input type="email" name="email" placeholder="Email" required>
                    <input type="text" name="address" placeholder="Street Address" required>
                    <input type="text" name="city" placeholder="City" required>
                    <input type="text" name="state" placeholder="State" required>
                    <input type="text" name="zip" placeholder="ZIP Code" required>
                </fieldset>
                <fieldset>
                    <legend>Payment Information</legend>
                    <input type="text" name="card_number" placeholder="Card Number" required>
                    <input type="text" name="expiry" placeholder="MM/YY" required>
                    <input type="text" name="cvv" placeholder="CVV" required>
                </fieldset>
                <button type="submit">Complete Purchase</button>
            </form>
        </section>
    </div>
</body>
</html>"#,
    )
}
