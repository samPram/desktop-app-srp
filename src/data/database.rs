use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::sync::{Arc, Mutex};
use thiserror::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbConnection = Arc<Mutex<SqliteConnection>>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    
    #[error("Diesel error: {0}")]
    DieselError(#[from] diesel::result::Error),
    
    #[error("Migration error: {0}")]
    MigrationError(String),
    
    #[error("Environment variable error: {0}")]
    EnvError(#[from] env::VarError),
}

pub fn establish_connection() -> Result<DbConnection, DatabaseError> {
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "data/dyno_test_history.db".to_string());
    
    // Ensure the database directory exists
    if let Some(parent) = std::path::Path::new(&database_url).parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|e| {
            log::warn!("Failed to create database directory: {}", e);
        });
    }
    
    let mut conn = SqliteConnection::establish(&database_url)
        .map_err(|e| DatabaseError::ConnectionError(format!("Failed to connect to database: {}", e)))?;
    
    // Run pending migrations
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| DatabaseError::MigrationError(format!("{:?}", e)))?;
    
    log::info!("Database connection established at: {}", database_url);
    Ok(Arc::new(Mutex::new(conn)))
}