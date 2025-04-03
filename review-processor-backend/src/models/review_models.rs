use serde::{Deserialize, Serialize};

/// Request model for creating a new review
///
/// This struct represents the data required to create a new review.
/// It is used as the request body for review creation endpoints.
#[derive(Serialize, Deserialize)]
pub struct CreateReviewRequest {
    /// The rating given in the review (typically on a scale)
    pub rating: i16,
    /// The text content of the review or suggestion
    pub suggestion: String,
}

/// Database model representing a review in the system
///
/// This struct maps to the database table 'suggestions' and represents
/// a complete review record with all its fields.
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Review {
    /// The text content of the review or suggestion
    #[sqlx(rename="suggestion")]
    pub suggestion: String,
    /// The rating given in the review (typically on a scale)
    #[sqlx(rename="rating")]
    pub rating: i16,
    /// The email address of the user who created the review
    #[sqlx(rename="created_by")]
    pub created_by: String
}