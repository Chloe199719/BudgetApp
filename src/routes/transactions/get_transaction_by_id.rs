use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    routes::users::logout::session_user_id,
    types::{general::ErrorResponse, transactions::create::TransactionOutcomeWithReceipt},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionId {
    pub transaction_id: i32,
}

#[tracing::instrument(name = "Getting transaction by id", skip(pool, session))]
#[get("/get_transaction_by_id/{transaction_id}")]
pub async fn get_transaction_by_id(
    pool: Data<PgPool>,
    session: actix_session::Session,
    transaction_id: Path<TransactionId>,
) -> HttpResponse {
    //Validate user is logged in and get user id
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
    match get_transaction_by_id_db(&session_uuid, transaction_id.transaction_id, &pool).await {
        Ok(transaction) => {
            tracing::event!(target: "session", tracing::Level::INFO, "Successfully got transaction from DB");
            return actix_web::HttpResponse::Ok().json(transaction);
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                tracing::event!(target: "session", tracing::Level::WARN, "Transaction not found");
                return actix_web::HttpResponse::NotFound().json(ErrorResponse {
                    error: "Transaction not found".to_string(),
                });
            }
            _ => {
                tracing::event!(target: "session", tracing::Level::ERROR, "Unable to get transaction from DB: {:#?}", e);
                return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Something unexpected happened. Kindly try again.".to_string(),
                });
            }
        },
    }
}

#[tracing::instrument(name = "Getting transaction by id from db", skip(pool))]
pub async fn get_transaction_by_id_db(
    user_id: &uuid::Uuid,
    transaction_id: i32,
    pool: &PgPool,
) -> Result<TransactionOutcomeWithReceipt, sqlx::Error> {
    let transactions = sqlx::query_as!(
        TransactionOutcomeWithReceipt,
        r#"
            SELECT
                transactions.transaction_id as transaction_id,
                amount,
                category_id,
                description,
                date,
                transaction_type as "transaction_type: _",
                receipt_id,
                transactions.user_id as user_id,
                currency as "currency: _",
                receipts.receipt_url as "receipt_url?"
            FROM
                transactions
            LEFT JOIN receipts ON transactions.receipt_id = receipts.id
            WHERE
                transactions.user_id = $1 AND
                transactions.transaction_id = $2
                and transactions.deleted = false
            "#,
        user_id,
        transaction_id
    )
    .fetch_one(pool)
    .await?;
    Ok(transactions)
}
