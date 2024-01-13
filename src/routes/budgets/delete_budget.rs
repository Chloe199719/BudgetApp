use actix_web::{
    delete,
    web::{Data, Path},
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    queries::budget::{check_budget_exists_db, delete_budget_db},
    routes::users::logout::session_user_id,
    types::general::{ErrorResponse, SuccessResponse},
    utils::constant::BACK_END_TARGET,
};

#[derive(Debug, Deserialize)]
pub struct DeleteBudgetPath {
    pub budget_id: i32,
}

#[tracing::instrument(name = "Delete Budget", skip(pool, session))]
#[delete("/delete/{budget_id}")]
pub async fn delete_budget_route(
    pool: Data<PgPool>,
    path: Path<DeleteBudgetPath>,
    session: actix_session::Session,
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
    match check_budget_exists_db(&pool, path.budget_id, session_uuid).await {
        Ok(exists) => {
            if !exists {
                tracing::event!(target: BACK_END_TARGET, tracing::Level::WARN, "Budget does not exist");
                return actix_web::HttpResponse::NotFound().json(ErrorResponse {
                    error: "Budget does not exist".to_string(),
                });
            }
        }
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if budget exists: {}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to check if budget exists".to_string(),
            });
        }
    }

    let mut db_transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to connect to database: {}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to connect to database".to_string(),
            });
        }
    };

    match delete_budget_db(&mut db_transaction, path.budget_id, session_uuid).await {
        Ok(_) => match db_transaction.commit().await {
            Ok(_) => {
                tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Successfully deleted budget");
                actix_web::HttpResponse::Ok().json(SuccessResponse {
                    message: "Budget deleted successfully".to_string(),
                })
            }
            Err(e) => {
                tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to commit transaction: {}", e);
                actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to commit transaction".to_string(),
                })
            }
        },
        Err(e) => match db_transaction.rollback().await {
            Ok(_) => {
                tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to delete budget: {}", e);
                actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to delete budget".to_string(),
                })
            }
            Err(e) => {
                tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to rollback transaction: {}", e);
                actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Failed to rollback transaction".to_string(),
                })
            }
        },
    }
}
