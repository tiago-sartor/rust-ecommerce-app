use axum::{
    Json,
    response::{Html},
};
use serde::{Deserialize, Serialize};

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

pub fn get_mock_cart() -> Cart {
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
