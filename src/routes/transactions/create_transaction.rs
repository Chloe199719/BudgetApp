use actix_web::{post, web::Data, HttpResponse};

use sqlx::PgPool;

use crate::{
    routes::users::logout::session_user_id,
    types::{
        general::ErrorResponse,
        transactions::create::{
            CreateTransactionRequest, Transaction, TransactionCurrency, TransactionOutcome,
            TransactionType,
        },
    },
    uploads::client::Client,
};

#[tracing::instrument(name = "Creating a transaction", skip(form, pool, session, s3_client))]
#[post("/create")]
pub async fn create_transaction(
    pool: Data<PgPool>,
    form: actix_multipart::form::MultipartForm<CreateTransactionRequest>,
    session: actix_session::Session,
    s3_client: Data<Client>,
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

    //Create a transaction object

    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to begin DB transaction: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happened. Kindly try again.".to_string(),
            });
        }
    };

    todo!("Implement this endpoint.")
}

#[tracing::instrument(name = "Save transaction in DB", skip(pool))]
pub async fn save_transaction_without_recipe(
    transaction_data: Transaction,
    user_id: uuid::Uuid,
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<TransactionOutcome, sqlx::Error> {
    let transaction = sqlx::query_as!(
        TransactionOutcome,
        r#"
        INSERT INTO transactions (amount, category_id, description, date, transaction_type, user_id, currency)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        returning transaction_id, amount, category_id, description, date, transaction_type as "transaction_type: _", receipt_id, user_id , currency as "currency: _" ;
        "#,
        &transaction_data.amount,
        &transaction_data.category_id.unwrap(),
        &transaction_data.description,
        &transaction_data.transaction_date,
        &transaction_data.transaction_type as &TransactionType,
        user_id,
        transaction_data.currency as Option<TransactionCurrency>,
    )
    .fetch_one(pool.as_mut())
    .await?;
    Ok(transaction)
    // todo!("Implement this function.")
}
