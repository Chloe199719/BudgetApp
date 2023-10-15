#[derive(serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SuccessResponse {
    pub message: String,
}
