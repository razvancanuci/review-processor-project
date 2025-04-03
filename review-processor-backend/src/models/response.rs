use serde::{Deserialize, Serialize};

/// Generic response model for API endpoints
///
/// This struct provides a standardized way to format API responses.
/// It can be used to wrap any type of data and includes success/failure
/// status and optional messages.
///
/// # Type Parameters
///
/// * `T` - The type of data being returned in the response
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseModel<T> {
    /// Indicates whether the operation was successful
    success: bool,
    /// The actual data being returned (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<T>,
    /// An optional message providing additional information
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl<T> ResponseModel<T> {
    /// Creates a failure response
    ///
    /// This method creates a response indicating that an operation failed.
    ///
    /// # Arguments
    ///
    /// * `message` - An optional message explaining the failure
    ///
    /// # Returns
    ///
    /// Returns a `ResponseModel` with:
    /// * `success` set to `false`
    /// * `value` set to `None`
    /// * `message` set to the provided message (if any)
    pub fn failure(message: Option<String>) -> Self {
        ResponseModel {
            success: false,
            value: None,
            message,
        }
    }

    /// Creates a success response
    ///
    /// This method creates a response indicating that an operation succeeded.
    ///
    /// # Arguments
    ///
    /// * `value` - The data to be returned in the response
    ///
    /// # Returns
    ///
    /// Returns a `ResponseModel` with:
    /// * `success` set to `true`
    /// * `value` set to the provided data
    /// * `message` set to `None`
    pub fn success(value: Option<T>) -> Self {
        ResponseModel {
            success: true,
            value,
            message: None,
        }
    }
}