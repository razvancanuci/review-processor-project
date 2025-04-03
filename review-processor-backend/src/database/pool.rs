use std::env;
use sqlx::{ PgPool };
use crate::models::errors::DatabaseError;

/// Creates a new PostgreSQL database connection pool.
///
/// This function reads the `DATABASE_URL` environment variable and uses it to
/// establish a connection pool to the PostgreSQL database.
///
/// # Returns
///
/// Returns a `Result` containing:
/// - `Ok(PgPool)`: A PostgreSQL connection pool on success
/// - `Err(DatabaseError)`: An error if the connection fails or if the `DATABASE_URL` is not set
///
/// # Panics
///
/// This function will panic if the `DATABASE_URL` environment variable is not set.
///
/// # Examples
///
/// ```rust
/// use review_processor_backend::database::pool;
///
/// #[tokio::main]
/// async fn main() {
///     let pool = pool::create_database_connection().await.unwrap();
///     // Use the pool for database operations
/// }
/// ```
pub async fn create_database_connection() -> Result<PgPool, DatabaseError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|err| DatabaseError(err.to_string()))?;

    Ok(pool)
}