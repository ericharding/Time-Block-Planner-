mod auth;
mod db;
mod handlers;
mod models;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "time_block_planner=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database
    let db_pool = db::init_db().await?;
    tracing::info!("Database initialized");

    // Build application routes
    let app = Router::new()
        // Auth routes
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/logout", post(handlers::auth::logout))
        .route("/api/auth/me", get(handlers::auth::me))

        // Plan routes
        .route("/api/plans", get(handlers::plans::list_plans))
        .route("/api/plans/:date", get(handlers::plans::get_plan))
        .route("/api/plans", post(handlers::plans::create_plan))
        .route("/api/plans/:id", put(handlers::plans::update_plan))
        .route("/api/plans/:id", delete(handlers::plans::delete_plan))

        // Column routes (interruptions)
        .route("/api/columns", post(handlers::columns::create_column))
        .route("/api/columns/:id", delete(handlers::columns::delete_column))

        // Time block routes
        .route("/api/blocks", post(handlers::blocks::create_block))
        .route("/api/blocks/:id", put(handlers::blocks::update_block))
        .route("/api/blocks/:id", delete(handlers::blocks::delete_block))

        // Serve static files (frontend)
        .nest_service("/", ServeDir::new("frontend/dist").append_index_html_on_directories(true))

        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(db_pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
