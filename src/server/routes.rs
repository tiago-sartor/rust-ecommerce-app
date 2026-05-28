use crate::middlewares;
use crate::server::handlers::backend::{auth as admin_auth, catalog as admin_catalog, orders as admin_orders, system as admin_system};
use crate::server::handlers::frontend::{auth as frontend_auth, cart as frontend_cart, catalog as frontend_catalog, customer as frontend_customer};
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
            .route("/login", get(admin_auth::admin_login_get).post(admin_auth::admin_login_post))
            .route("/logout", get(admin_auth::admin_logout))
            .route(
                "/forgot-password",
                get(admin_auth::admin_forgot_password_get).post(admin_auth::admin_forgot_password_post),
            )
            .route(
                "/reset-password/{token}",
                get(admin_auth::admin_reset_password_get).post(admin_auth::admin_reset_password_post),
            ),
    )
}

pub fn protected_admin_routes(pool: PgPool) -> Router<PgPool> {
    Router::new().nest(
        "/admin",
        Router::<PgPool>::new()
            .route("/dashboard", get(admin_system::admin_dashboard))
            .route("/orders", get(admin_orders::admin_orders))
            .route("/add-order", get(admin_orders::admin_add_order_get))
            .route("/order-details/{id}", get(admin_orders::admin_order_details))
            .route("/customers", get(admin_orders::admin_customers))
            .route(
                "/add-customer",
                get(admin_orders::admin_add_customer_get)//.post(admin_orders::admin_add_customer_post),
            )
            //.route("/customer-details/{id}", get(admin_orders::admin_customer_details))
            .route("/products", get(admin_catalog::admin_products))
            .route("/add-product", get(admin_catalog::admin_add_product))
            .route("/edit-product/{id}", get(admin_catalog::admin_edit_product))
            .route("/account", get(admin_system::admin_account))
            .route("/settings", get(admin_system::admin_settings))
            .nest(
                "/emails",
                Router::new()
                    .route("/", get(admin_system::admin_emails))
                    .route("/bulk-actions", post(admin_system::admin_email_bulk_actions))
                    .route("/{id}/details", get(admin_system::admin_email_details))
                    .route("/{id}/resend", post(admin_system::admin_email_resend))
                    .route("/{id}/delete", post(admin_system::admin_email_delete)),
            )
            .layer(middleware::from_fn_with_state(pool, middlewares::auth::admin_auth)),
    )
}

pub fn frontend_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(frontend_catalog::home_page))
        .route("/home", get(frontend_catalog::home_page))
        .route("/products", get(frontend_catalog::catalog_page))
        .route("/products/{slug}", get(frontend_catalog::product_detail_page))
        .route("/cart", get(frontend_cart::cart_page))
        .route("/checkout", get(frontend_cart::checkout_page))
        .route("/login", get(frontend_auth::login_page).post(frontend_auth::customer_login_post))
        .route("/logout", get(frontend_auth::customer_logout))
}

pub fn protected_customer_routes(pool: PgPool) -> Router<PgPool> {
    Router::new().nest(
        "/my-account",
        Router::<PgPool>::new()
            .route("/", get(frontend_customer::customer_account))
            .route("/edit-account", get(frontend_customer::customer_edit_account))
            .route("/change-password", get(frontend_customer::customer_password))
            .route("/orders", get(frontend_customer::customer_orders))
            .route("/order-details", get(frontend_customer::customer_order_details))
            .route("/addresses", get(frontend_customer::customer_address))
            .route("/edit-address", get(frontend_customer::customer_edit_address))
            .route("/wishlist", get(frontend_customer::customer_wishlist))
            .layer(middleware::from_fn_with_state(pool, middlewares::auth::customer_auth)),
    )
}
