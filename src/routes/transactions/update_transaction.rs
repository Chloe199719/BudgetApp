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
    types::{general::ErrorResponse, transactions::create::TransactionCurrency},
    uploads::client::Client,
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
    pub receipt: Option<String>
}

#[tracing::instrument(name = "Updating a transaction", skip(form, pool, session, s3_client))]
#[patch("/transactions/{transaction_id}/update")]
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

   let transaction = match get_transaction_by_id_db(&session_uuid,path.transaction_id, &pool).await {
        Ok(transaction) => transaction,
        Err(e) => {
            match e {
                sqlx::Error::RowNotFound => {
                    tracing::event!(target: "session", tracing::Level::ERROR, "Transaction not found");
                    return actix_web::HttpResponse::NotFound().json(ErrorResponse {
                        error: "Transaction not found".to_string(),
                    });
                }
                _ => {
                    tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get transaction from database: {}", e);
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
        //TODO: Delete old receipt if it exists
        let s3_key_prefix = format!("receipts/{}/{}/", session_uuid,path.transaction_id);
        let upload_file = s3_client.upload(receipt, &s3_key_prefix).await;

        transactions_object.receipt = Some(upload_file.s3_url);
    }


    todo!("Implement update transaction")
}


#[tracing::instrument(name = "Updating transaction on database", skip(pool ))]
async fn update_transaction_db(
    pool: &PgPool,
    transaction_id: i32,
    object: ObjectTransaction,
) -> Result<(), sqlx::Error> {
    todo!("Implement update transaction on database");
}


