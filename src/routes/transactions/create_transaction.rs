use actix_multipart::form;
use actix_web::{post, web::Data, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

use crate::{
    routes::users::logout::session_user_id, types::general::ErrorResponse, uploads::client::Client,
};

#[derive(form::MultipartForm)]
pub struct CreateTransactionRequest {
    pub amount: form::text::Text<i64>,
    pub category_id: Option<form::text::Text<i32>>,
    pub description: form::text::Text<String>,
    pub transaction_date: form::text::Text<chrono::NaiveDate>,
    pub transaction_type: form::text::Text<TransactionType>,
    pub currency: Option<form::text::Text<TransactionCurrency>>,
    #[multipart(limit = "1 MiB")]
    pub receipt: Option<form::tempfile::TempFile>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
}
impl std::str::FromStr for TransactionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(Self::Deposit),
            "WITHDRAWAL" => Ok(Self::Withdrawal),
            _ => Err(format!("{} is not a valid transaction type.", s)),
        }
    }
}
impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deposit => write!(f, "DEPOSIT"),
            Self::Withdrawal => write!(f, "WITHDRAWAL"),
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "currencys_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionCurrency {
    EUR,
    USD,
    WON,
    YEN,
    POUND,
}
impl std::str::FromStr for TransactionCurrency {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EUR" => Ok(Self::EUR),
            "USD" => Ok(Self::USD),
            "WON" => Ok(Self::WON),
            "YEN" => Ok(Self::YEN),
            "POUND" => Ok(Self::POUND),
            _ => Err(format!("{} is not a valid currency type.", s)),
        }
    }
}
impl std::fmt::Display for TransactionCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EUR => write!(f, "EUR"),
            Self::USD => write!(f, "USD"),
            Self::WON => write!(f, "WON"),
            Self::YEN => write!(f, "YEN"),
            Self::POUND => write!(f, "POUND"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct TransactionOutcome {
    pub transaction_id: i32,
    pub amount: f64,
    pub category_id: i32,
    pub description: String,
    pub date: chrono::DateTime<Utc>,
    pub transaction_type: String,
    pub receipt_id: Option<i32>,
    pub user_id: uuid::Uuid,
    pub currency: TransactionCurrency,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub amount: f64,
    pub category_id: Option<i32>,
    pub description: String,
    pub transaction_date: chrono::DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub receipt_id: Option<String>,
    pub currency: Option<TransactionCurrency>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Receipt {
    pub user_id: uuid::Uuid,
    pub transaction_id: i32,
    pub url: String,
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
