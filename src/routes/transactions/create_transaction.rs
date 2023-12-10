use actix_multipart::form;

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
