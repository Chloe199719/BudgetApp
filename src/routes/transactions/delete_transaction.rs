use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    routes::users::logout::session_user_id,
    types::general::{ErrorResponse, SuccessResponse},
    utils::constant::BACK_END_TARGET,
};

#[derive(Deserialize, Debug)]
struct DeleteTransaction {
    transaction_id: i32,
}

#[tracing::instrument(name = "Delete Transaction", skip(pool, session))]
#[actix_web::delete("/delete/{transaction_id}")]
pub async fn delete_transaction(
    pool: Data<PgPool>,
    path: Path<DeleteTransaction>,
    session: actix_session::Session,
) -> HttpResponse {
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

    match delete_transaction_db(&pool, path.transaction_id, session_uuid).await {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            message: "Transaction deleted successfully".to_string(),
        }),
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => {
                    tracing::event!(target: BACK_END_TARGET , tracing::Level::WARN, "Transaction not found: {}", e);
                    return HttpResponse::NotFound().json(ErrorResponse {
                        error: "Transaction not found".to_string(),
                    });
                }
                _ => {}
            }

            tracing::event!(target: "delete_transaction", tracing::Level::ERROR, "Failed to delete transaction: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to delete transaction".to_string(),
            })
        }
    }
}

#[tracing::instrument(name = "Delete Transaction in DB", skip(pool))]
async fn delete_transaction_db(
    pool: &PgPool,
    transaction_id: i32,
    user_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    match sqlx::query!(
        r#"
        UPDATE transactions
        SET deleted = true
        WHERE transaction_id = $1 AND user_id = $2
        returning transaction_id
       "#,
        transaction_id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(transaction) => {
            if transaction.transaction_id != transaction_id {
                return Err(sqlx::Error::RowNotFound);
            }
        }
        Err(e) => return Err(e),
    };
    Ok(())
}
