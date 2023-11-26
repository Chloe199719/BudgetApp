use actix_web::{ HttpResponse, web::{ Path, Data }, get };
use serde::{ Deserialize, Serialize };
use sqlx::PgPool;

use crate::{
    types::general::ErrorResponse,
    routes::users::logout::session_user_id,
    utils::constant::BACK_END_TARGET,
    queries::category::check_category_exists_return_it,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct PathCategory {
    pub category_id: i32,
}

#[tracing::instrument(name = "Get category by id", skip(pool, session, data))]
#[get("/get/{category_id}")]
pub async fn get_category_by_id(
    pool: Data<PgPool>,
    session: actix_session::Session,
    data: Path<PathCategory>
) -> HttpResponse {
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(
                target: "session",
                tracing::Level::ERROR,
                "Failed to get user from session. User unauthorized: {}",
                e
            );
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again".to_string(),
            });
        }
    };
    match check_category_exists_return_it(&pool, data.category_id, session_uuid).await {
        Ok(category) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Successfully got category by id");
            HttpResponse::Ok().json(category)
        }
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to get category by id: {:#?}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Category does not exist".to_string(),
            })
        }
    }
}
