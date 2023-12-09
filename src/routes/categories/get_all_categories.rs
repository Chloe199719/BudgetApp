use crate::{
    queries::category::get_all_categories_by_user_id, routes::users::logout::session_user_id,
    types::general::ErrorResponse, utils::constant::BACK_END_TARGET,
};
use actix_web::{get, web::Data, HttpResponse};
use sqlx::PgPool;

#[tracing::instrument(name = "Get category by id", skip(pool, session))]
#[get("/get")]
pub async fn get_all_category_by_user_id(
    pool: Data<PgPool>,
    session: actix_session::Session,
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
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            });
        }
    };

    match get_all_categories_by_user_id(&pool, session_uuid).await {
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
