use actix_web::web::Data;

use crate::{
    routes::users::logout::session_user_id,
    types::{categories::Category, general::ErrorResponse},
    utils::constant::BACK_END_TARGET,
};
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateCategory {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 3, max = 500))]
    pub description: String,
}

#[tracing::instrument(name = "Creating a category", skip(pool, session))]
#[actix_web::post("/create")]
pub async fn create_category(
    pool: Data<PgPool>,
    session: actix_session::Session,
    data: Json<CreateCategory>,
) -> actix_web::HttpResponse {
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return actix_web::HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            });
        }
    };
    match create_category_in_db(&pool, &data.name, &data.description, session_uuid).await {
        Ok(category) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Successfully created category: {:#?}", category);
            return actix_web::HttpResponse::Ok().json(category);
        }
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to create category: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to create category. Kindly try again.".to_string(),
            });
        }
    }
}

#[tracing::instrument(name = "Creating a category in DB", skip(pool))]
async fn create_category_in_db(
    pool: &PgPool,
    name: &str,
    description: &str,
    user_id: uuid::Uuid,
) -> Result<Category, sqlx::Error> {
    match sqlx::query_as!(
        Category,
        r#"
            INSERT INTO categories (category_name, description, user_id)
            VALUES ($1, $2, $3)
            RETURNING *;
        "#,
        name,
        description,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(e) => {
            tracing::event!(target: "sqlx", tracing::Level::INFO, "Successfully created category: {:#?}", e);
            Ok(e)
        }
        Err(e) => {
            tracing::event!(target: "sqlx", tracing::Level::ERROR, "Failed to create category: {:#?}", e);
            Err(e)
        }
    }
}
