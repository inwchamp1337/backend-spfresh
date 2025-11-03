use serde::{Deserialize, Serialize};

/// Request to add a new review
#[derive(Debug, Deserialize)]
pub struct AddReviewRequest {
    pub review_title: String,
    pub review_body: String,
    pub product_id: String,
    pub review_rating: u8,
}

/// Response after adding a review
#[derive(Debug, Serialize)]
pub struct AddReviewResponse {
    pub vector_id: usize,
    pub status: String,
    pub message: String,
}

/// Request to search for similar reviews
#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    
    #[serde(default = "default_top_k")]
    pub top_k: usize,
}

fn default_top_k() -> usize {
    10
}

/// A single search result
#[derive(Debug, Serialize)]
pub struct SearchResultItem {
    pub review_title: String,
    pub review_body: String,
    pub product_id: String,
    pub review_rating: u8,
    pub similarity_score: f32,
    pub vector_id: usize,
}

/// Response from search endpoint
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
    pub total_found: usize,
    pub query: String,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub total_reviews: usize,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl AddReviewRequest {
    /// Validate the request
    pub fn validate(&self) -> Result<(), String> {
        if self.review_title.trim().is_empty() {
            return Err("Review title cannot be empty".to_string());
        }
        if self.review_body.trim().is_empty() {
            return Err("Review body cannot be empty".to_string());
        }
        if self.product_id.trim().is_empty() {
            return Err("Product ID cannot be empty".to_string());
        }
        if self.review_rating < 1 || self.review_rating > 5 {
            return Err("Review rating must be between 1 and 5".to_string());
        }
        Ok(())
    }
}

impl SearchRequest {
    /// Validate the request
    pub fn validate(&self) -> Result<(), String> {
        if self.query.trim().is_empty() {
            return Err("Query cannot be empty".to_string());
        }
        if self.top_k == 0 || self.top_k > 100 {
            return Err("top_k must be between 1 and 100".to_string());
        }
        Ok(())
    }
}
