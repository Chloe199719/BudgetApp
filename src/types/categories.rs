use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
    pub description: String,
    pub user_id: Uuid,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub is_default: bool,
    pub budget_id: Option<i32>,
    pub start_date: Option<DateTime<chrono::Utc>>,
    pub end_date: Option<DateTime<chrono::Utc>>,
    pub recurring: Option<bool>,
    pub amount: Option<f64>,
}
