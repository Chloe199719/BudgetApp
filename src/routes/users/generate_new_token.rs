use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct UserEmail {
    email: String,
}

pub struct SimpleUser {
    id: Uuid,
    email: String,
    display_name: String,
    unique_name: String,
    is_active: bool,
    is_staff: bool,
    is_superuser: bool,
    thumbnail: Option<String>,
    data_joined: chrono::DateTime<chrono::Utc>,
}

// #[tracing::instrument(name = "Regenerate token for a user", skip(pool, redis_pool))]
// #[actix_web::post("/regenerate-token/")]
