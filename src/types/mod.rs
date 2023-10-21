use chrono::DateTime;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

pub mod tokens;
pub mod general;
#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub display_name: String,
    pub unique_name: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub thumbnail: Option<String>,
    pub data_joined: DateTime<chrono::Utc>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVisible {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub unique_name: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub thumbnail: Option<String>,
    pub data_joined: DateTime<chrono::Utc>,
}
