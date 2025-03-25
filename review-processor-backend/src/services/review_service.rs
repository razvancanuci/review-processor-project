use async_trait::async_trait;
use log::{error, warn};
use sentry::{capture_error, capture_event};
use sentry::protocol::Event;
use crate::models::errors::ApiError;
use crate::models::review_models::{CreateReviewRequest, Review};
use crate::repositories::review_repository::ReviewRepositoryTrait;

#[async_trait]
pub trait ReviewServiceTrait: Send + Sync {
    async fn create_review(&self, review: CreateReviewRequest, created_by: String) -> Result<Review, ApiError>;
}

pub struct ReviewService {
    repository: Box<dyn ReviewRepositoryTrait>
}


impl ReviewService {
    pub fn new(repository: Box<dyn ReviewRepositoryTrait>) -> Self {
        Self {
            repository
        }
    }
    
    fn validate_input(review: &CreateReviewRequest) -> Result<(), &'static str> {
        if review.rating < 1 || review.rating > 10 {
            return Err("Rating must be between 1 and 10");
        }
        if !review.suggestion.is_empty() && review.suggestion.len() < 50 {
            return Err("Suggestion must be more than 50 characters");
        }
        Ok(())
    }
}

#[async_trait]
impl ReviewServiceTrait for ReviewService {
    async fn create_review(&self, review: CreateReviewRequest, created_by: String) -> Result<Review, ApiError> {
        Self::validate_input(&review)
            .map_err(|e| {
                
                capture_event(Event {
                    message: Some(format!("Invalid input: {} by {}", e, created_by)),
                    level: sentry::Level::Warning,
                    ..Default::default()
                });
                warn!("Invalid input: {} by {}", e, created_by);
                ApiError::BadRequest(e)
            })?;
        
        let review_already = self.repository.get_by_email(&created_by)
            .await.map_err(|e| {
                capture_error(&e);
                error!("Failed to get review: {}", e);
                ApiError::InternalServerError
            })?;
        
        if review_already.is_some() {
            
            capture_event(Event {
                message: Some(format!("Review twice attempted by {}", created_by)),
                level: sentry::Level::Warning,
                ..Default::default()
            });
            
            warn!("Review twice attempted by {}", created_by);
            return Err(ApiError::BadRequest("Review already submitted"));
        }
        
        let review = Review {
            rating: review.rating,
            suggestion: review.suggestion,
            created_by
        };

        self.repository.create_review(review)
            .await
            .map_err(|e| {
                capture_error(&e);
                error!("Failed to create review: {}", e);
                ApiError::InternalServerError
            })
    }
}


#[cfg(test)]
mod tests {
    use mockall::predicate::always;
    use crate::repositories::review_repository::MockReviewRepositoryTrait;
    use crate::services::review_service::ReviewServiceTrait;

    fn setup_mocks() -> MockReviewRepositoryTrait {
        let repository = MockReviewRepositoryTrait::new();
        repository
    }
    
    #[tokio::test]
    async fn create_review_validation_fail_returns_error() {
        let service = super::ReviewService::new(Box::new(setup_mocks()));
        let result = service.create_review(super::CreateReviewRequest {
            rating: 0,
            suggestion: "Test suggestion".to_string()
        }, "user".to_string()).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_review_already_reviewed_returns_error() {
        let mut repository = setup_mocks();
        repository.expect_get_by_email()
            .with(always())
            .returning(|_| Box::pin(async { Ok(Some(super::Review {
                rating: 5,
                suggestion: "Test suggestion".to_string(),
                created_by: "user".to_string()
            })) }));
        let service = super::ReviewService::new(Box::new(repository));
        let result = service.create_review(super::CreateReviewRequest {
            rating: 10,
            suggestion: "".to_string()
        }, "user".to_string()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_review_returns_ok() {
        let mut repository = setup_mocks();
        repository.expect_get_by_email()
            .with(always())
            .returning(|_| Box::pin(async { Ok(None)}));
        repository.expect_create_review()
            .with(always())
            .returning(|_| Box::pin(async { Ok(super::Review {
                rating: 10,
                suggestion: "".to_string(),
                created_by: "user".to_string()
            }) }));
        let service = super::ReviewService::new(Box::new(repository));
        let result = service.create_review(super::CreateReviewRequest {
            rating: 10,
            suggestion: "".to_string()
        }, "user".to_string()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn create_review_invalid_rating_returns_error() {
        let service = super::ReviewService::new(Box::new(setup_mocks()));
        let result = service.create_review(super::CreateReviewRequest {
            rating: 0,
            suggestion: "Test suggestion".to_string()
        }, "user".to_string()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn create_review_short_suggestion_returns_error() {
        let service = super::ReviewService::new(Box::new(setup_mocks()));
        let result = service.create_review(super::CreateReviewRequest {
            rating: 5,
            suggestion: "Short".to_string()
        }, "user".to_string()).await;

        assert!(result.is_err());
    }
}