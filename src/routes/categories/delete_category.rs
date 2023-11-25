use actix_web::{ delete, HttpResponse, web::{ Path, Data } };
use serde::{ Deserialize, Serialize };
use sqlx::{ PgPool, Postgres, Transaction };

use crate::{
    routes::users::logout::session_user_id,
    types::general::{ ErrorResponse, SuccessResponse },
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
    data: Path<DeleteCategory>
) -> HttpResponse {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: "discord_backend", tracing::Level::ERROR, "Unable to begin DB transaction: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happened. Kindly try again.".to_string(),
            });
        }
    };
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again".to_string(),
            });
        }
    };
    match check_category_exists(&mut transaction, data.category_id, session_uuid).await {
        Ok(true) => (),
        Ok(false) => {
            transaction.rollback().await.expect("Failed to rollback transaction");
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Category does not exist");
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Category does not exist".to_string(),
            });
        }
        Err(e) => {
            transaction.rollback().await.expect("Failed to rollback transaction");
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists: {:#?}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to delete category. Kindly try again.".to_string(),
            });
        }
    }

    match delete_category_in_db(&mut transaction, data.category_id, session_uuid).await {
        Ok(_) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Successfully deleted category");
            match transaction.commit().await.is_ok() {
                true => (),
                false => {
                    tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to commit transaction");
                    return HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Failed to delete category. Kindly try again.".to_string(),
                    });
                }
            }
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

#[tracing::instrument(name = "Deleting a category in DB", skip(transaction))]
async fn delete_category_in_db(
    transaction: &mut Transaction<'_, Postgres>,
    category_id: i32,
    user_id: uuid::Uuid
) -> Result<(), sqlx::Error> {
    match
        sqlx::query!(
                r#"
                    DELETE FROM categories
                    WHERE category_id = $1 AND user_id = $2
            "#,
                category_id,
                user_id
            )
            .execute(transaction.as_mut()).await
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
#[rustfmt::skip]

#[tracing::instrument(name = "Check if category exists", skip(transaction))]
async fn check_category_exists (
    transaction: &mut Transaction<'_, Postgres>,
    category_id: i32,
    user_id: uuid::Uuid
) -> Result<bool, sqlx::Error> {
    match
        sqlx::query!(
                r#"
                    SELECT EXISTS(
                        SELECT 1 FROM categories
                        WHERE category_id = $1 AND user_id = $2
                    )
            "#,
                category_id,
                user_id
            )
            .fetch_one(transaction.as_mut()).await
    {
        Ok(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::INFO, "Successfully checked if category exists");
            Ok(e.exists.unwrap_or(false))
        },
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists in DB: {:#?}", e);
            Err(e)
        }
    }
}
