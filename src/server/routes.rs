use crate::middlewares;
use crate::server::handlers::{backend, frontend};
use axum::response::Redirect;
use axum::{
    Router, middleware,
    routing::{get, post},
};
use sqlx::PgPool;

pub fn admin_routes() -> Router<PgPool> {
    Router::new().nest(
        "/admin",
        Router::new()
            .route("/", get(async || Redirect::to("/admin/login")))
            .route("/login", get(backend::admin_login_get).post(backend::admin_login_post))
            .route("/logout", get(backend::admin_logout))
            .route(
                "/forgot-password",
                get(backend::admin_forgot_password_get).post(backend::admin_forgot_password_post),
            )
            .route(
                "/reset-password/{token}",
                get(backend::admin_reset_password_get).post(backend::admin_reset_password_post),
            ),
    )
}

pub fn protected_admin_routes(pool: PgPool) -> Router<PgPool> {
    Router::new().nest(
        "/admin",
        Router::<PgPool>::new()
            .route("/dashboard", get(backend::admin_dashboard))
            .route("/orders", get(backend::admin_orders))
            .route("/add-order", get(backend::admin_add_order_get))
            .route("/order-details/{id}", get(backend::admin_order_details))
            .route("/customers", get(backend::admin_customers))
            .route("/add-customer", get(backend::admin_add_customer_get).post(backend::admin_add_customer_post))
            //.route("/customer-details/{id}", get(backend::admin_customer_details))
            .route("/products", get(backend::admin_products))
            .route("/add-product", get(backend::admin_add_product))
            .route("/edit-product/{id}", get(backend::admin_edit_product))
            .route("/account", get(backend::admin_account))
            .route("/settings", get(backend::admin_settings))
            .nest(
                "/emails",
                Router::new()
                    .route("/", get(backend::admin_emails))
                    .route("/bulk-actions", post(backend::admin_email_bulk_actions))
                    .route("/{id}/details", get(backend::admin_email_details))
                    .route("/{id}/resend", post(backend::admin_email_resend))
                    .route("/{id}/delete", post(backend::admin_email_delete)),
            )
            .layer(middleware::from_fn_with_state(pool, middlewares::auth::admin_auth)),
    )
}

pub fn frontend_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(frontend::home_page))
        .route("/home", get(frontend::home_page))
        .route("/products", get(frontend::catalog_page))
        .route("/products/{slug}", get(frontend::product_detail_page))
        .route("/cart", get(frontend::cart_page))
        .route("/checkout", get(frontend::checkout_page))
        .route("/login", get(frontend::login_page).post(frontend::customer_login_post))
        .route("/logout", get(frontend::customer_logout))
}

pub fn protected_customer_routes(pool: PgPool) -> Router<PgPool> {
    Router::new().nest(
        "/my-account",
        Router::<PgPool>::new()
            .route("/", get(frontend::customer_account))
            .route("/edit-account", get(frontend::customer_edit_account))
            .route("/change-password", get(frontend::customer_password))
            .route("/orders", get(frontend::customer_orders))
            .route("/order-details", get(frontend::customer_order_details))
            .route("/addresses", get(frontend::customer_address))
            .route("/edit-address", get(frontend::customer_edit_address))
            .route("/wishlist", get(frontend::customer_wishlist))
            .layer(middleware::from_fn_with_state(pool, middlewares::auth::customer_auth)),
    )
}
