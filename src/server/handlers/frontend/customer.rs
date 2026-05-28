use axum::response::Html;

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
