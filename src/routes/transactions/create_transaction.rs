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
    utils::transactions::utils::{
        get_users_default_category, get_users_default_currency, save_recipe_url,
        save_transaction_without_recipe,
    },
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

    //Generate transaction_struct object
    let mut transaction_struct = Transaction {
        amount: form.amount.0,
        category_id: None,
        description: form.description.0.clone(),
        transaction_date: form.transaction_date.0,
        receipt_id: None,
        transaction_type: form.transaction_type.0.clone(),
        currency: None,
    };

    //Check if category_id and currency are provided
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

    //Save transaction
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

    //Save recipe if provided
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

    //Commit transaction if no recipe provided and return transaction
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
