use std::env;
use sqlx::{ PgPool };
use crate::models::errors::DatabaseError;

pub async fn create_database_connection() -> Result<PgPool, DatabaseError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|err| DatabaseError(err.to_string()))?;

    Ok(pool)
}