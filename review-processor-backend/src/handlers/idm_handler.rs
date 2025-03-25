use actix_web::{get, HttpResponse};

#[get("/sso")]
pub async fn single_sign_on() -> HttpResponse {
    HttpResponse::Ok().finish()
} 