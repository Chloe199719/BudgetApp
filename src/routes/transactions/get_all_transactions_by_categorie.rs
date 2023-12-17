use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    routes::users::logout::session_user_id,
    types::{general::ErrorResponse, transactions::create::TransactionOutcomeWithReceipt},
    utils::constant::BACK_END_TARGET,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllTransactionsByCategoriesRequest {
    pub category_id: Option<i32>,
}

#[tracing::instrument(name = "Getting all transactions by category", skip(pool, session))]
#[get("/get_all_transactions_by_category")]
pub async fn get_all_transactions_by_category(
    pool: Data<PgPool>,
    session: actix_session::Session,
    query: Query<GetAllTransactionsByCategoriesRequest>,
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
    match query.category_id {
        Some(category_id) => {
            match get_all_transactions_by_categories_db(&session_uuid, &category_id, &pool).await {
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
        None => match get_all_transactions_by_categories_default_db(&session_uuid, &pool).await {
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
        },
    }
}

#[tracing::instrument(
    name = "Get All Transactions By Categories default from db",
    skip(pool)
)]
pub async fn get_all_transactions_by_categories_default_db(
    user_id: &uuid::Uuid,
    pool: &PgPool,
) -> Result<Vec<TransactionOutcomeWithReceipt>, sqlx::Error> {
    let transactions = sqlx::query_as!(
        TransactionOutcomeWithReceipt,
        r#"
    SELECT
        transactions.transaction_id as transaction_id,
        amount,
        transactions.category_id,
        transactions.description,
        date,
        transaction_type as "transaction_type: _",
        receipt_id,
        transactions.user_id as user_id,
        currency as "currency: _",
        receipts.receipt_url as "receipt_url?"
    from
        transactions
        LEFT JOIN users ON transactions.user_id = users.id
        LEFT JOIN categories ON transactions.category_id = categories.category_id
        LEFT JOIN receipts ON transactions.receipt_id = receipts.id
    WHERE
        users.id = $1
        and categories.is_default = TRUE
        "#,
        user_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(transactions)
}

#[tracing::instrument(name = "Get All Transactions By Categories from db", skip(pool))]
pub async fn get_all_transactions_by_categories_db(
    user_id: &uuid::Uuid,
    category_id: &i32,
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
        transactions.user_id = $1 AND transactions.category_id = $2
    ORDER BY date DESC;
        "#,
        user_id,
        category_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(transactions)
}
