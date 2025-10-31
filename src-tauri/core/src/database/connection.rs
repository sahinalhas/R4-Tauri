use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use log::{info, error};
use std::path::PathBuf;

/// Initialize database connection and run migrations
pub async fn initialize_database(db_path: PathBuf) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    // Create directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create database directory: {}", e))?;
    }
    
    let db_url = format!("sqlite:{}", db_path.display());
    
    info!("Initializing database at: {}", db_url);
    
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    // Run migrations
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            error!("Migration failed: {}", e);
            format!("Migration failed: {}", e)
        })?;
    
    info!("Database initialized successfully");
    
    // Enable WAL mode for better concurrency
    sqlx::query("PRAGMA journal_mode = WAL")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to set WAL mode: {}", e))?;
    
    // Set other performance optimizations
    sqlx::query("PRAGMA synchronous = NORMAL")
        .execute(&pool)
        .await
        .ok();
    
    sqlx::query("PRAGMA cache_size = -64000")
        .execute(&pool)
        .await
        .ok();
    
    sqlx::query("PRAGMA temp_store = MEMORY")
        .execute(&pool)
        .await
        .ok();
    
    Ok(pool)
}
