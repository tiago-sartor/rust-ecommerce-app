use axum::{Router, routing::get};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

use rust_ecommerce_app::server::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    sqlx::migrate!("./src/migrations").run(&pool).await.expect("Failed to run migrations");

    let session_store = PostgresStore::new(pool.clone());
    let session_layer = SessionManagerLayer::new(session_store).with_expiry(Expiry::OnInactivity(time::Duration::weeks(1)));

    let app = Router::new()
        // Frontend routes
        .merge(routes::frontend_routes())
        .nest("/my-account", routes::protected_customer_routes())
        // Admin routes
        .nest("/admin", routes::protected_admin_routes())
        .route("/admin/login", get(backend_handlers::admin_login_get).post(backend_handlers::admin_login_post))
        .route("/admin/logout", get(backend_handlers::admin_logout))
        .route("/admin/forgot-password", get(backend_handlers::admin_forgot_password_get).post(backend_handlers::admin_forgot_password_post))
        .route("/admin/reset-password", get(backend_handlers::admin_reset_password_get).post(backend_handlers::admin_reset_password_post))
        // Static files
        .nest_service("/public", tower_http::services::ServeDir::new("public"))
        .layer(session_layer)
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("listening on http://{address}");
    axum::serve(listener, app).await.unwrap();
}
