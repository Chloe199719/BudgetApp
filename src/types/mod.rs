use chrono::DateTime;
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

pub mod tokens;
pub mod general;
pub mod upload;
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
    pub profile: UserProfile,
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
    pub profile: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub phone_number: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub github_link: Option<String>,
    pub about_me: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_link: Option<String>,
}
