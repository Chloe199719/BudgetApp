use actix_multipart::form;
use actix_web::{
    patch,
    web::{Data, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    routes::{users::logout::session_user_id, transactions::get_transaction_by_id::get_transaction_by_id_db},
    types::{general::ErrorResponse, transactions::create::{TransactionCurrency, TransactionOutcomeWithReceipt}},
    uploads::client::Client, utils::constant::BACK_END_TARGET,
};

#[derive(Deserialize, Debug)]
pub struct PathUpdate {
    pub transaction_id: i32,
}

#[derive(form::MultipartForm)]
pub struct UpdateTransaction {
    pub description: Option<form::text::Text<String>>,
    pub amount: Option<form::text::Text<f64>>,
    pub currency: Option<form::text::Text<TransactionCurrency>>,
    #[multipart(limit = "1 MiB")]
    pub receipt: Option<form::tempfile::TempFile>,
}
#[derive(Deserialize,Serialize, Debug)]
pub struct ObjectTransaction {
    pub description: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<TransactionCurrency>,
    pub receipt: Option<i32>
}

#[tracing::instrument(name = "Updating a transaction", skip(form, pool, session, s3_client))]
#[patch("/{transaction_id}/update")]
pub async fn update_transaction_route(
    pool: Data<PgPool>,
    path: Path<PathUpdate>,
    form: actix_multipart::form::MultipartForm<UpdateTransaction>,
    s3_client: Data<Client>,
    session: actix_session::Session,
) -> HttpResponse {

    let session_uuid = match session_user_id(&session).await {
        Ok(session_uuid) => session_uuid,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return actix_web::HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            });
        }
    };

    let mut db_transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to connect to database: {}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to connect to database".to_string(),
            });
        }
    };
   let transaction = match get_transaction_by_id_db(&session_uuid,path.transaction_id, &pool).await {
        Ok(transaction) => transaction,
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => {
                    tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Transaction not found");
                    return actix_web::HttpResponse::NotFound().json(ErrorResponse {
                        error: "Transaction not found".to_string(),
                    });
                }
                _ => {
                    tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to get transaction from database: {}", e);
                    return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Failed to get transaction from database".to_string(),
                    });
                }
            }
        }
    };

    let mut transactions_object = ObjectTransaction {
        description: None,
        amount: None,
        currency: None,
        receipt: None,
    };
    if let Some(description) = &form.description {
        transactions_object.description = Some(description.0.clone());
    }
        
    if let Some(amount) = &form.amount {
        transactions_object.amount = Some(amount.0.clone());
    }
    if let Some(currency) = &form.currency {
        transactions_object.currency = Some(currency.0.clone());
    }

    if let Some(receipt) = &form.0.receipt{
        let s3_key_prefix = format!("receipts/{}/{}/", session_uuid,path.transaction_id);
        let upload_file = s3_client.upload(receipt, &s3_key_prefix).await;

        let receipt_id = match transaction.receipt_id {
            Some(receipt_id) => {
                if let Some(receipt_url) = &transaction.receipt_url {
                    let s3_image_key = &receipt_url[receipt_url.find("receipts").unwrap_or(receipt_url.len())..];
                    if !s3_client.delete_file(s3_image_key).await {
                        tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "Failed to delete old receipt from s3"); 
                    }
                }
                match update_transaction_db_receipt(&mut db_transaction, receipt_id, path.transaction_id, session_uuid, upload_file.s3_url.clone()).await {
                    Ok(id) => id,
                    Err(e) => {
                        tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to update transaction receipt on database: {}", e);
                        return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Failed to update transaction receipt on database".to_string(),
                        });
                    }
                }
            }
            None => {
                match store_transaction_receipt_db(&mut db_transaction, path.transaction_id, session_uuid, upload_file.s3_url.clone()).await {
                    Ok (id) => id,
                    Err(e) => {
                        tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to store transaction receipt on database: {}", e);
                        return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                            error: "Failed to store transaction receipt on database".to_string(),
                        });
                    }
                }
            }
        };
        transactions_object.receipt = Some(receipt_id);
    }
    match update_transaction_db(&mut db_transaction, path.transaction_id, &session_uuid, transactions_object).await {
        Ok(transaction) => {
            match db_transaction.commit().await {
                Ok(_) => {
                    return actix_web::HttpResponse::Ok().json(transaction);
                }
                Err(e) => {
                    tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to commit transaction to database: {}", e);
                    return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "Failed to commit transaction to database".to_string(),
                    });
                }
            }
        }
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to update transaction on database: {}", e);
            return actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to update transaction on database".to_string(),
            });
        }
    }

}


#[tracing::instrument(name = "Updating transaction on database", skip(pool ))]
async fn update_transaction_db(
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    transaction_id: i32,
    user_id: &uuid::Uuid,
    object: ObjectTransaction,
) -> Result<TransactionOutcomeWithReceipt, sqlx::Error> {
    match sqlx::query!(
        r#"
        UPDATE transactions
        SET 
            description = COALESCE($1, description),
            amount = COALESCE($2,amount),
            currency = COALESCE($3, currency),
            receipt_id = COALESCE($6, receipt_id)
        WHERE 
            transaction_id = $4 AND user_id = $5
        "#,
        object.description,
        object.amount,
        object.currency as Option<TransactionCurrency>,
        transaction_id,
        user_id,
        object.receipt
    ).execute(pool.as_mut()).await {
        Ok(_) => match sqlx::query_as!(
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
            ).fetch_one(pool.as_mut()).await {
                Ok(transaction) => Ok(transaction),
                Err(e) => Err(e)
            }
        Err(e) => Err(e)
    }
}

#[tracing::instrument(name = "Store transaction receipt on database", skip(pool ))]
async fn store_transaction_receipt_db(
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    transaction_id: i32,
    user_id: uuid::Uuid,
    receipt_url: String,
) -> Result<i32, sqlx::Error> {
    match sqlx::query!(
        r#"
        INSERT INTO receipts (transaction_id, user_id, receipt_url)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        transaction_id,
        user_id,
        receipt_url
    ).fetch_one(pool.as_mut()).await {
        Ok(receipt) => Ok(receipt.id),
        Err(e) => Err(e)
    }
}

#[tracing::instrument(name = "Update transaction receipt on database", skip(pool ))]
async fn update_transaction_db_receipt(
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    receipt_id: i32,
    transaction_id: i32,
    user_id: uuid::Uuid,
    receipt_url: String,
) -> Result<i32, sqlx::Error> {
    match sqlx::query!(
        r#"
        UPDATE receipts
        SET receipt_url = $1
        WHERE id = $2 AND transaction_id = $3 AND user_id = $4
        RETURNING id
        "#,
        receipt_url,
        receipt_id,
        transaction_id,
        user_id
    ).fetch_one(pool.as_mut()).await {
        Ok(receipt) => Ok(receipt.id),
        Err(e) => Err(e)
    }
}

