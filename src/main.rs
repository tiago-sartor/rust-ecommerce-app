use axum::{Router, middleware};
use sqlx::PgPool;
use tokio::{signal, task::AbortHandle};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer, cookie::SameSite};
use tower_sessions_sqlx_store::PostgresStore;
use tracing_subscriber::fmt::format::FmtSpan;

use rust_ecommerce_app::{middlewares::csrf::csrf_middleware, server::routes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    sqlx::migrate!("./src/migrations").run(&pool).await.expect("Failed to run migrations");

    // === SESSION STORE ===
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(session_store.clone().continuously_delete_expired(tokio::time::Duration::from_hours(1)));

    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("SID")
        .with_http_only(true)
        .with_secure(true)
        .with_same_site(SameSite::Strict)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    // === ROUTES ===
    let app = Router::new()
        // Frontend routes
        .merge(routes::frontend_routes())
        .merge(routes::protected_customer_routes(pool.clone()))
        // Admin routes
        .merge(routes::admin_routes())
        .merge(routes::protected_admin_routes(pool.clone()))
        // Static files
        .nest_service("/assets", ServeDir::new("public"))
        // Middlewares
        .layer(middleware::from_fn(csrf_middleware))
        .layer(session_layer)
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("listening on http://{address}");
    // Ensure we use a shutdown signal to abort the deletion task.
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle())).await?;

    deletion_task.await??;

    Ok(())
}

fn init_tracing() {
    // simple console + file example
    let fmt = tracing_subscriber::fmt().with_span_events(FmtSpan::CLOSE).with_target(false);

    // console
    fmt.with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate()).expect("Failed to install signal handler").recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
