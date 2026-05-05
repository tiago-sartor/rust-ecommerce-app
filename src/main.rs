use axum::{
    Router,
    routing::get,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

use rust_ecommerce_app::server::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./src/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        // frontend routes
        .route("/", get(frontend_handlers::home_page))
        .route("/products", get(frontend_handlers::products_page))
        .route(
            "/products/{slug}",
            get(frontend_handlers::product_detail_page),
        )
        .route("/cart", get(frontend_handlers::cart_page))
        .route("/checkout", get(frontend_handlers::checkout_page))
        // admin routes
        .route(
            "/admin/dashboard",
            get(backend_handlers::admin_dashboard), //.layer(from_fn(auth::admin_auth)),
        )
        // static files
        .nest_service("/public", tower_http::services::ServeDir::new("public"))
        .layer(CorsLayer::permissive());

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("listening on http://{address}");
    axum::serve(listener, app).await.unwrap();
}
