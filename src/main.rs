mod handlers;
mod router;
mod utils;

use crate::router::router;
use axum::extract::FromRef;
use axum_flash::Key;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    flash_config: axum_flash::Config,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load environment variables
    dotenv().ok();

    // Read database URL from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Error connecting to database");

    // println!("Successfully connected to PostgreSQL!");

    // let result: (i32,) = sqlx::query_as("SELECT 1 + 1").fetch_one(&pool).await?;

    // println!("1 + 1 = {}", result.0);

    // adding db to the global state

    let app_state = AppState {
        pool,
        flash_config: axum_flash::Config::new(Key::generate()),
    };

    impl FromRef<AppState> for axum_flash::Config {
        fn from_ref(state: &AppState) -> axum_flash::Config {
            state.flash_config.clone()
        }
    }
    
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    let app = router()
        .nest_service("/assets", serve_dir.clone())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
