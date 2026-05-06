use axum::{
    Router,
    routing::get,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tower_sessions::{SessionManagerLayer, Expiry};
use tower_sessions_sqlx_store::PostgresStore;

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

    let session_store = PostgresStore::new(pool.clone());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    let admin_routes = Router::new()
        .route("/dashboard", get(backend_handlers::admin_dashboard))
        .route("/edit-account", get(backend_handlers::admin_account))
        .layer(axum::middleware::from_fn(rust_ecommerce_app::middlewares::auth::admin_auth));

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
        .route("/login", get(frontend_handlers::home_page).post(frontend_handlers::customer_login_post))
        .route("/logout", get(frontend_handlers::customer_logout))
        
        // admin routes
        .nest("/admin", admin_routes)
        .route("/admin/login", get(backend_handlers::admin_login_get).post(backend_handlers::admin_login_post))
        .route("/admin/logout", get(backend_handlers::admin_logout))

        // static files
        .nest_service("/public", tower_http::services::ServeDir::new("public"))
        .layer(session_layer)
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("listening on http://{address}");
    axum::serve(listener, app).await.unwrap();
}
