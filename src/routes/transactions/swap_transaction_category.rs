use actix_web::{web::{Data, Path}, HttpResponse, patch};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{routes::users::logout::session_user_id, types::general::{ErrorResponse, SuccessResponse}, utils::constant::BACK_END_TARGET};

#[derive(Debug, Deserialize)]
pub struct PathSwapTransactionCategory {
    pub transaction_id: i32,
    pub category_id: i32,
}

#[tracing::instrument(name = "Swapping transaction category", skip(pool, session))]
#[patch("/swap_category/{transaction_id}/{category_id}")]
pub async fn swap_transaction_category(
    pool: Data<PgPool>,
    session: actix_session::Session,
    path: Path<PathSwapTransactionCategory>,
    ) -> HttpResponse {
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again".to_string(),
            });
        }
    };
    match check_category(&pool, &session_uuid, path.category_id).await {
        Ok(_) => {
            match swap_transaction_category_db(&pool, path.transaction_id, path.category_id, &session_uuid).await {
                Ok(_) => HttpResponse::Ok().json(SuccessResponse {
                    message: "Transaction category swapped successfully".to_string(),
                }),
                Err(e) => {
                    match e {
                        sqlx::Error::RowNotFound => {
                            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Transaction does not exist or is not owned by user: {}", e);
                            HttpResponse::BadRequest().json(ErrorResponse {
                                error: "Transaction does not exist or is not owned by user".to_string(),
                            })
                        },
                        _ => {
                            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to swap transaction category: {}", e);
                            HttpResponse::InternalServerError().json(ErrorResponse {
                                error: "Failed to swap transaction category".to_string(),
                            })
                        }
                    }
                }
            }
        },
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => {
                    tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Category does not exist or is not owned by user: {}", e);
                    HttpResponse::BadRequest().json(ErrorResponse {
                        error: "Category does not exist or is not owned by user".to_string(),
                    })
                },
                _ => {
                    tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to swap transaction category: {}", e);
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Failed to swap transaction category".to_string(),
                    })
                }
            }
        }
    }
}

#[tracing::instrument(name = "Check if category exits and is owned by user", skip(pool))]
async fn check_category(pool: &PgPool, session_uuid: &uuid::Uuid, category_id: i32) -> Result<(), sqlx::Error> {
    let category = sqlx::query!("SELECT category_id FROM categories WHERE category_id = $1 AND user_id = $2", category_id, session_uuid)
        .fetch_optional(pool)
        .await?;
    match category {
        Some(_) => Ok(()),
        None => Err(sqlx::Error::RowNotFound),
    }
}

#[tracing::instrument(name = "Swap transaction category", skip(pool))]
async fn swap_transaction_category_db(pool: &PgPool, transaction_id: i32, category_id: i32 , user_id: &uuid::Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE transactions SET category_id = $1 WHERE transaction_id = $2 AND user_id = $3", category_id, transaction_id, user_id)
        .execute(pool)
        .await?;
    Ok(())
}
