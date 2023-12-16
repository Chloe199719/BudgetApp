use actix_multipart::form;
use actix_web::{post, web::Data, HttpResponse};
use chrono::Utc;

use sqlx::PgPool;

use crate::{
    routes::users::logout::session_user_id,
    types::{
        general::ErrorResponse,
        transactions::create::{
            Transaction, TransactionCurrency, TransactionOutcome, TransactionOutcomeWithReceipt,
            TransactionType,
        },
    },
    uploads::client::Client,
};

#[derive(form::MultipartForm)]
pub struct CreateTransactionRequest {
    pub amount: form::text::Text<f64>,
    pub category_id: Option<form::text::Text<i32>>,
    pub description: form::text::Text<String>,
    pub transaction_date: form::text::Text<chrono::DateTime<Utc>>,
    pub transaction_type: form::text::Text<TransactionType>,
    pub currency: Option<form::text::Text<TransactionCurrency>>,
    #[multipart(limit = "1 MiB")]
    pub receipt: Option<form::tempfile::TempFile>,
}

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

    let mut transaction_struct = Transaction {
        amount: form.amount.0,
        category_id: None,
        description: form.description.0.clone(),
        transaction_date: form.transaction_date.0,
        receipt_id: None,
        transaction_type: form.transaction_type.0.clone(),
        currency: None,
    };

    if let Some(category_id) = &form.category_id {
        transaction_struct.category_id = Some(category_id.0);
    } else {
        transaction_struct.category_id = match get_users_default_category(
            &session_uuid,
            &mut transaction,
        )
        .await
        {
            Ok(category_id) => Some(category_id),
            Err(e) => {
                tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to get default category: {:#?}", e);
                return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Something unexpected happened. Kindly try again.".to_string(),
                });
            }
        }
    }

    if let Some(currency) = &form.currency {
        transaction_struct.currency = Some(currency.0.clone());
    } else {
        transaction_struct.currency = match get_users_default_currency(
            &session_uuid,
            &mut transaction,
        )
        .await
        {
            Ok(currency) => Some(currency),
            Err(e) => {
                tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to get default currency: {:#?}", e);
                return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Something unexpected happened. Kindly try again.".to_string(),
                });
            }
        }
    }

    let save_transaction = match save_transaction_without_recipe(
        transaction_struct,
        &session_uuid,
        &mut transaction,
    )
    .await
    {
        Ok(transaction) => transaction,
        Err(e) => {
            transaction.rollback().await.unwrap();
            tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to save transaction: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happened. Kindly try again.".to_string(),
            });
        }
    };

    if let Some(recipe) = &form.0.receipt {
        let s3_key_prefix = format!(
            "receipts/{}/{}",
            session_uuid, save_transaction.transaction_id
        );
        let upload_file = s3_client.upload(recipe, &s3_key_prefix).await;

        match save_recipe_url(
            save_transaction.transaction_id,
            &upload_file.s3_url,
            &session_uuid,
            &mut transaction,
        )
        .await
        {
            Ok(transaction_outcome) => match transaction.commit().await {
                Ok(_) => {
                    tracing::event!(target: "backend", tracing::Level::INFO, "Transaction committed successfully");
                    return actix_web::HttpResponse::Ok().json(transaction_outcome);
                }
                Err(e) => {
                    tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to commit transaction: {:#?}", e);
                    return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Something unexpected happened. Kindly try again.".to_string(),
                    });
                }
            },
            Err(e) => {
                transaction.rollback().await.unwrap();
                tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to save recipe url: {:#?}", e);
                return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Something unexpected happened. Kindly try again.".to_string(),
                });
            }
        }
    }

    match transaction.commit().await {
        Ok(_) => {
            tracing::event!(target: "backend", tracing::Level::INFO, "Transaction committed successfully");
            HttpResponse::Ok().json(save_transaction)
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to commit transaction: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something unexpected happened. Kindly try again.".to_string(),
            });
        }
    }
}

#[tracing::instrument(name = "Save transaction in DB", skip(pool))]
pub async fn save_transaction_without_recipe(
    transaction_data: Transaction,
    user_id: &uuid::Uuid,
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
        TransactionType::DEPOSIT as TransactionType,
        user_id,
        transaction_data.currency.unwrap() as TransactionCurrency,
    )
    .fetch_one(pool.as_mut())
    .await?;
    Ok(transaction)
}

#[tracing::instrument(name = "Save recipe url in DB", skip(pool))]
pub async fn save_recipe_url(
    transaction_id: i32,
    url: &str,
    user_id: &uuid::Uuid,
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<TransactionOutcomeWithReceipt, sqlx::Error> {
    let recipe = sqlx::query!(
        r#"
        INSERT INTO receipts (transaction_id, receipt_url, user_id)
        VALUES ($1, $2, $3)
        RETURNING id;
        "#,
        transaction_id,
        url,
        user_id,
    )
    .fetch_one(pool.as_mut())
    .await?;

    let transaction_update = sqlx::query_as!(
        TransactionOutcome,
        r#"
        UPDATE transactions
        SET receipt_id = $1
        WHERE transaction_id = $2
    
        returning transaction_id, amount, category_id, description, date, transaction_type as "transaction_type: _", receipt_id, user_id , currency as "currency: _" ;
        "#,
        recipe.id,
        transaction_id,
    )
    .fetch_one(pool.as_mut())
    .await?;
    let transaction_with_recipe = TransactionOutcomeWithReceipt {
        transaction_id: transaction_update.transaction_id,
        amount: transaction_update.amount,
        category_id: transaction_update.category_id,
        description: transaction_update.description,
        date: transaction_update.date,
        transaction_type: transaction_update.transaction_type,
        receipt_id: transaction_update.receipt_id,
        receipt_url: Some(url.to_string()),
        user_id: transaction_update.user_id,
        currency: transaction_update.currency,
    };
    Ok(transaction_with_recipe)
}

#[tracing::instrument(name = "Get usersDefault Category", skip(pool))]
pub async fn get_users_default_category(
    user_id: &uuid::Uuid,
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<i32, sqlx::Error> {
    let category = sqlx::query!(
        r#"
        SELECT category_id
        FROM categories
        WHERE user_id = $1 AND is_default = true;
        "#,
        user_id,
    )
    .fetch_one(pool.as_mut())
    .await?;
    Ok(category.category_id)
}

#[tracing::instrument(name = "Get User Default Currency", skip(pool))]
pub async fn get_users_default_currency(
    user_id: &uuid::Uuid,
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<TransactionCurrency, sqlx::Error> {
    let currency = sqlx::query_as!(
        CurrencyReturn,
        r#"
        SELECT currency as "currency: _"
        FROM user_profile
        WHERE user_id = $1
       
        "#,
        user_id,
    )
    .fetch_one(pool.as_mut())
    .await?;
    Ok(currency.currency)
}

pub struct CurrencyReturn {
    pub currency: TransactionCurrency,
}
