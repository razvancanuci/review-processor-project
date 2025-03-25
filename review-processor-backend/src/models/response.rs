use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseModel<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl<T> ResponseModel<T> {
    pub fn failure(message: Option<String>) -> Self {
        ResponseModel {
            success: false,
            value: None,
            message,
        }
    }
    pub fn success(value: Option<T>) -> Self {
        ResponseModel {
            success: true,
            value,
            message: None,
        }
    }
}