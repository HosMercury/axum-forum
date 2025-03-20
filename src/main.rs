mod auth_middleware;
mod filters;
mod handlers;
mod models;
mod router;
mod utils;

use crate::router::router;
use axum_messages::MessagesManagerLayer;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tower_http::services::{ServeDir, ServeFile};
use tower_sessions::cookie::time::Duration;
use tower_sessions::session_store::ExpiredDeletion;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Read database URL from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Error connecting to database");

    //// session
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60 * 60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(60 * 60 * 24)));

    // println!("Successfully connected to PostgreSQL!");

    // let result: (i32,) = sqlx::query_as("SELECT 1 + 1").fetch_one(&pool).await?;

    // println!("1 + 1 = {}", result.0);

    // adding db to the global state

    let app_state = AppState { pool };

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    let app = router()
        .layer(MessagesManagerLayer) // MUST BE BEFORE SESSION LAYER
        .layer(session_layer)
        .nest_service("/assets", serve_dir.clone())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    deletion_task.await??;

    Ok(())
}
