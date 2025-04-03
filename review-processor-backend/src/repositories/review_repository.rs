use async_trait::async_trait;
use sqlx::{PgPool};
use crate::database::pool::create_database_connection;
use crate::models::errors::DatabaseError;
use crate::models::review_models::Review;
use mockall::automock;

/// Trait defining the interface for review repository operations
///
/// This trait provides an abstraction layer for review data access operations.
/// It is marked with `#[automock]` to allow for easy testing with mock implementations.
#[async_trait]
#[automock]
pub trait ReviewRepositoryTrait: Send + Sync {
    /// Creates a new review in the database
    ///
    /// # Arguments
    ///
    /// * `review` - The review data to be stored
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// * `Ok(Review)` - The created review with database-generated fields
    /// * `Err(DatabaseError)` - If the database operation fails
    async fn create_review(&self, review: Review) -> Result<Review, DatabaseError>;

    /// Retrieves a review by the creator's email
    ///
    /// # Arguments
    ///
    /// * `email` - The email address of the review creator
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// * `Ok(Some(Review))` - If a review is found
    /// * `Ok(None)` - If no review is found
    /// * `Err(DatabaseError)` - If the database operation fails
    async fn get_by_email(&self, email: &str) -> Result<Option<Review>, DatabaseError>;
}

/// Implementation of the review repository using PostgreSQL
///
/// This struct holds a connection pool to the PostgreSQL database
/// and implements the `ReviewRepositoryTrait` for database operations.
pub struct ReviewRepository {
    pool: PgPool
}

impl ReviewRepository {
    /// Creates a new instance of ReviewRepository
    ///
    /// This constructor establishes a connection to the database
    /// using the connection pool.
    ///
    /// # Returns
    ///
    /// Returns a new `ReviewRepository` instance
    ///
    /// # Panics
    ///
    /// This function will panic if it cannot establish a database connection
    pub async fn new() -> Self {
        let pool = create_database_connection().await.expect("Failed to create connection pool");
        Self {
            pool
        }
    }
}

#[async_trait]
impl ReviewRepositoryTrait for ReviewRepository {
    /// Implementation of review creation
    ///
    /// Inserts a new review into the 'suggestions' table and returns
    /// the created record with any database-generated fields.
    async fn create_review(&self, review: Review) -> Result<Review, DatabaseError> {
        let result = sqlx::query_as::<_, Review>(
            r#"
            INSERT INTO suggestions (rating, suggestion, created_by)
            VALUES ($1, $2, $3)
            RETURNING rating, suggestion, created_by
            "#,
        )
        .bind(review.rating)
        .bind(review.suggestion)
        .bind(review.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| DatabaseError(e.to_string()))?;
        
        Ok(result)
    }

    /// Implementation of review retrieval by email
    ///
    /// Queries the 'suggestions' table for a review created by the specified email.
    async fn get_by_email(&self, email: &str) -> Result<Option<Review>, DatabaseError> {
        let review = sqlx::query_as::<_, Review>(
            "SELECT suggestion, rating, created_by FROM suggestions WHERE created_by = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DatabaseError(e.to_string()))?;

        Ok(review)
    }
}