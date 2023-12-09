use actix_web::{
    delete,
    web::{Data, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    queries::category::{check_category_exists, check_category_exists_return_it},
    routes::users::logout::session_user_id,
    types::general::{ErrorResponse, SuccessResponse},
    utils::constant::BACK_END_TARGET,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteCategory {
    pub category_id: i32,
}

#[tracing::instrument(name = "Deleting a category", skip(pool, session))]
#[delete("/delete/{category_id}")]
pub async fn delete_category(
    pool: Data<PgPool>,
    session: actix_session::Session,
    data: Path<DeleteCategory>,
) -> HttpResponse {
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            });
        }
    };
    match check_category_exists_return_it(&pool, data.category_id, session_uuid).await {
        Ok(category) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::DEBUG, "CHECK CATEGORY {:#?}",category);
            println!("{:?}", category);
            if category.is_default {
                tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Tried to delete default category");
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "You cannot delete your default category".to_string(),
                });
            }
        }
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists: {:#?}", e);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Category does not exist.".to_string(),
            });
        }
    }

    match delete_category_in_db(&pool, data.category_id, session_uuid).await {
        Ok(_) => {
            return HttpResponse::Ok().json(SuccessResponse {
                message: "Successfully deleted category".to_string(),
            });
        }
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to delete category: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to delete category. Kindly try again.".to_string(),
            });
        }
    }
}
#[rustfmt::skip]

#[tracing::instrument(name = "Deleting a category in DB", skip(pool))]
async fn delete_category_in_db(
    pool: &PgPool,
    category_id: i32,
    user_id: uuid::Uuid
) -> Result<(), sqlx::Error> {
    match
        sqlx::query!(
                r#"
                    DELETE FROM categories
                    WHERE category_id = $1 AND user_id = $2 AND is_default = false
            "#,
                category_id,
                user_id
            )
            .execute(pool).await
    {
        Ok(_) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::INFO, "Successfully deleted category");
            Ok(())},
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to delete category in DB: {:#?}", e);
            Err(e)
        }
    }
}
