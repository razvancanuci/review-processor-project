use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
use crate::models::errors::FormatErrorTrait;
use crate::models::response::ResponseModel;
use crate::models::review_models::{CreateReviewRequest, Review};
use crate::repositories::review_repository::ReviewRepository;
use crate::services::review_service::{ReviewService, ReviewServiceTrait};

#[post("")]
pub async fn add_review(
    http_request: HttpRequest,
    review: web::Json<CreateReviewRequest>) -> HttpResponse {
    let user_email = http_request.extensions().get::<String>().unwrap().to_owned();
    let service = ReviewService::new(Box::new(ReviewRepository::new().await));
    let result = service.create_review(review.into_inner(), user_email).await;
    
   match  result { 
         Ok(review) => HttpResponse::Created().json(ResponseModel::<Review>::success(Some(review))),
         Err(e) => {

             let (status, e) = e.get_message_status();
             HttpResponse::build(status).json(ResponseModel::<String>::failure(Some(e)))
         }
   } 
}