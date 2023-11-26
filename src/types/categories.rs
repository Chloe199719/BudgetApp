use chrono::DateTime;
use serde::{ Deserialize, Serialize };
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
}
//TODO: add updated_at
