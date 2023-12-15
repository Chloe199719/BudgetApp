use actix_multipart::form;

use chrono::Utc;

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
