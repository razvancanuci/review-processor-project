use async_trait::async_trait;
use sqlx::{PgPool};
use crate::database::pool::create_database_connection;
use crate::models::errors::DatabaseError;
use crate::models::review_models::Review;
use mockall::automock;

#[async_trait]
#[automock]
pub trait ReviewRepositoryTrait: Send + Sync {
    async fn create_review(&self, review: Review) -> Result<Review, DatabaseError>;
    async fn get_by_email(&self, email: &str) -> Result<Option<Review>, DatabaseError>;
}


pub struct ReviewRepository {
    pool: PgPool
}

impl ReviewRepository {
    pub async fn new() -> Self {
        let pool = create_database_connection().await.expect("Failed to create connection pool");
        Self {
            pool
        }
    }
}

#[async_trait]
impl ReviewRepositoryTrait for ReviewRepository {
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

    async fn get_by_email(&self, email: &str) -> Result<Option<Review>, DatabaseError> {
        let review = sqlx::query_as::<_, Review>("SELECT suggestion, rating, created_by FROM suggestions WHERE created_by = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DatabaseError(e.to_string()))?;

        Ok(review)
    }
}