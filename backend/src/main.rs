//! Document Path Management API Server

use axum::routing::get;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use document_path_db::api::create_router;
use document_path_db::storage::init_db_pool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "document_path_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./data/documents.db".to_string());

    tracing::info!("Connecting to database: {}", database_url);
    let pool = init_db_pool(&database_url).await?;
    tracing::info!("Database initialized with WAL mode");

    // Run migrations
    tracing::info!("Running database migrations");
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Migrations completed");

    // Build router with API endpoints
    let app = create_router(pool.clone())
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive());

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Starting server on {}", addr);
    tracing::info!("API endpoints:");
    tracing::info!("  GET    /api/documents           - Get all documents");
    tracing::info!("  POST   /api/documents           - Create document (auto-generated)");
    tracing::info!("  POST   /api/documents/manual    - Create document (manual number)");
    tracing::info!("  GET    /api/documents/:id       - Get document by ID");
    tracing::info!("  GET    /api/documents/number/:number - Get document by number");
    tracing::info!("  PUT    /api/documents/:id/path  - Update document path");
    tracing::info!("  DELETE /api/documents/:id       - Delete document (logical)");
    tracing::info!("  GET    /api/documents/search    - Search documents");
    tracing::info!("  GET    /health                  - Health check");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
