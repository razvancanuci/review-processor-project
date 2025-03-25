use std::fmt::{Display, Formatter};
use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(PartialEq, Error, Debug)]
pub enum ApiError {
    NotFound(&'static str),
    BadRequest(&'static str),
    InternalServerError,
}

impl Display for ApiError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ApiError::BadRequest(message) => _f.write_str(message),
            ApiError::NotFound(message) => _f.write_str(message),
            _ => _f.write_str("Something went wrong"),
        }
    }
}

pub trait FormatErrorTrait {
    fn get_message_status(&self) -> (StatusCode, String);
}

impl FormatErrorTrait for ApiError {
    fn get_message_status(&self) -> (StatusCode, String) {
        match *self {
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.to_owned()),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message.to_owned()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_owned(),
            ),
        }
    }
}

#[derive(Debug, Error)]
pub struct DatabaseError(pub String);

impl Display for DatabaseError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("Database error")
    }
}


