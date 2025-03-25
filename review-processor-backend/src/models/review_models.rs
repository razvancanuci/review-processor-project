use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateReviewRequest {
    pub rating: i16,
    pub suggestion: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Review {
    #[sqlx(rename="suggestion")]
    pub suggestion: String,
    #[sqlx(rename="rating")]
    pub rating: i16,
    #[sqlx(rename="created_by")]
    pub created_by: String
}