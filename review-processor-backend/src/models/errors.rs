use std::fmt::{Display, Formatter};
use actix_web::http::StatusCode;
use thiserror::Error;

/// Enum representing different types of API errors that can occur in the application
///
/// This enum is used to standardize error handling across the application
/// and provide consistent error responses to clients.
#[derive(PartialEq, Error, Debug)]
pub enum ApiError {
    /// Error indicating that a requested resource was not found
    NotFound(&'static str),
    /// Error indicating that the client's request was invalid
    BadRequest(&'static str),
    /// Error indicating an unexpected server-side error
    InternalServerError,
}

impl Display for ApiError {
    /// Formats the error message for display
    ///
    /// # Arguments
    ///
    /// * `_f` - The formatter to write the error message to
    ///
    /// # Returns
    ///
    /// Returns a `std::fmt::Result` indicating success or failure of the formatting
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ApiError::BadRequest(message) => _f.write_str(message),
            ApiError::NotFound(message) => _f.write_str(message),
            _ => _f.write_str("Something went wrong"),
        }
    }
}

/// Trait for formatting errors into HTTP status codes and messages
///
/// This trait provides a standardized way to convert application errors
/// into appropriate HTTP responses.
pub trait FormatErrorTrait {
    /// Converts an error into an HTTP status code and message
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// * The appropriate HTTP status code
    /// * A human-readable error message
    fn get_message_status(&self) -> (StatusCode, String);
}

impl FormatErrorTrait for ApiError {
    /// Implementation of error formatting for ApiError
    ///
    /// Maps each ApiError variant to an appropriate HTTP status code
    /// and error message.
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

/// Error type for database-related errors
///
/// This struct wraps database error messages and provides a standardized
/// way to handle database errors throughout the application.
#[derive(Debug, Error)]
pub struct DatabaseError(pub String);

impl Display for DatabaseError {
    /// Formats the database error for display
    ///
    /// # Arguments
    ///
    /// * `fmt` - The formatter to write the error message to
    ///
    /// # Returns
    ///
    /// Returns a `std::fmt::Result` indicating success or failure of the formatting
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("Database error")
    }
}


