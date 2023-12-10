use actix_multipart::form;
use actix_web::{post, web::Data, HttpResponse};
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
    #[multipart(limit = "1 MiB")]
    pub receipt: Option<form::tempfile::TempFile>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
}
impl std::str::FromStr for TransactionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(Self::DEPOSIT),
            "WITHDRAWAL" => Ok(Self::WITHDRAWAL),
            _ => Err(format!("{} is not a valid transaction type.", s)),
        }
    }
}
impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEPOSIT => write!(f, "DEPOSIT"),
            Self::WITHDRAWAL => write!(f, "WITHDRAWAL"),
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub amount: i64,
    pub category_id: Option<i32>,
    pub description: String,
    pub transaction_date: chrono::NaiveDate,
    pub transaction_type: TransactionType,
    pub receipt: Option<String>,
    pub user_id: i32,
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
    todo!("Implement this endpoint.")
}
