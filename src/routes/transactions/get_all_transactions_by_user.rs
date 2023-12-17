use actix_web::{get, web::Data, HttpResponse};
use sqlx::PgPool;

use crate::{
    routes::users::logout::session_user_id,
    types::{general::ErrorResponse, transactions::create::TransactionOutcomeWithReceipt},
    utils::constant::BACK_END_TARGET,
};

#[tracing::instrument(name = "Get All Transactions By User", skip(pool, session))]
#[get("/get_all_transactions_by_user")]
pub async fn get_all_transactions_by_user(
    pool: Data<PgPool>,
    session: actix_session::Session,
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
    match get_all_transactions_by_user_db(&session_uuid, &pool).await {
        Ok(transactions) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Successfully got transactions from DB");
            return actix_web::HttpResponse::Ok().json(transactions);
        }
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Unable to get transactions from DB: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happened. Kindly try again.".to_string(),
            });
        }
    }
}

#[tracing::instrument(name = "Get All Transactions By User from db", skip(pool))]
pub async fn get_all_transactions_by_user_db(
    user_id: &uuid::Uuid,
    pool: &PgPool,
) -> Result<Vec<TransactionOutcomeWithReceipt>, sqlx::Error> {
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
                transactions.user_id = $1
            ORDER BY date DESC;
        "#,
        user_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(transactions)
}
