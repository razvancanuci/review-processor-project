use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use crate::models::errors::FormatErrorTrait;
use crate::models::response::ResponseModel;
use crate::models::review_models::{CreateReviewRequest, Review};
use crate::repositories::review_repository::ReviewRepository;
use crate::services::review_service::{ReviewService, ReviewServiceTrait};

/// Handler for creating a new review
///
/// This endpoint accepts a POST request with a review payload and creates a new review
/// in the system. The review is associated with the authenticated user's email.
///
/// # Arguments
///
/// * `http_request` - The HTTP request containing user authentication information
/// * `review` - The review data to be created, wrapped in a JSON payload
///
/// # Returns
///
/// Returns an HTTP response with:
/// * Status 201 (Created) and the created review on success
/// * An appropriate error status and message if the operation fails
///
/// # Errors
///
/// This function will return an error response if:
/// * The user is not authenticated
/// * The review data is invalid
/// * There are database or service-level errors
#[post("")]
pub async fn add_review(
    http_request: HttpRequest,
    review: web::Json<CreateReviewRequest>) -> HttpResponse {
    // Extract the authenticated user's email from the request
    let user_email = http_request.extensions().get::<String>().unwrap().to_owned();
    
    // Initialize the review service with a new repository instance
    let service = ReviewService::new(Box::new(ReviewRepository::new().await));
    
    // Attempt to create the review
    let result = service.create_review(review.into_inner(), user_email).await;
    
    // Handle the result and return appropriate HTTP response
    match result { 
        // On success, return 201 Created with the review data
        Ok(review) => HttpResponse::Created().json(ResponseModel::<Review>::success(Some(review))),
        // On error, return appropriate error status and message
        Err(e) => {
            let (status, e) = e.get_message_status();
            HttpResponse::build(status).json(ResponseModel::<String>::failure(Some(e)))
        }
    } 
}