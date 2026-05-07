use crate::middlewares;
use crate::server::{backend_handlers, frontend_handlers};
use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn protected_admin_routes() -> Router<PgPool> {
    Router::new()
        .route("/dashboard", get(backend_handlers::admin_dashboard))
        // .route("/orders", get(backend_handlers::admin_orders))
        // .route("/add-order", get(backend_handlers::admin_add_order))
        // .route("/order-details", get(backend_handlers::admin_order_details))
        // .route("/products", get(backend_handlers::admin_products))
        // .route("/add-product", get(backend_handlers::admin_add_product))
        // .route("/edit-product", get(backend_handlers::admin_edit_product))
        // .route("/edit-account", get(backend_handlers::admin_account))
        // .route("/settings", get(backend_handlers::admin_settings))
        .layer(axum::middleware::from_fn(middlewares::auth::admin_auth))
}

pub fn protected_customer_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(frontend_handlers::customer_account))
        .route("/edit-account", get(frontend_handlers::customer_edit_account))
        .route("/change-password", get(frontend_handlers::customer_password))
        .route("/orders", get(frontend_handlers::customer_orders))
        .route("/order-details", get(frontend_handlers::customer_order_details))
        .route("/addresses", get(frontend_handlers::customer_address))
        .route("/edit-address", get(frontend_handlers::customer_edit_address))
        .route("/wishlist", get(frontend_handlers::customer_wishlist))
        .layer(axum::middleware::from_fn(middlewares::auth::customer_auth))
}

pub fn frontend_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(frontend_handlers::home_page))
        .route("/home", get(frontend_handlers::home_page))
        .route("/products", get(frontend_handlers::catalog_page))
        .route("/products/{slug}", get(frontend_handlers::product_detail_page))
        .route("/cart", get(frontend_handlers::cart_page))
        .route("/checkout", get(frontend_handlers::checkout_page))
        .route("/login", get(frontend_handlers::login_page).post(frontend_handlers::customer_login_post))
        .route("/logout", get(frontend_handlers::customer_logout))
}
