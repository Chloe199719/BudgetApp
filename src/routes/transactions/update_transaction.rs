use actix_multipart::form;
use serde::Deserialize;

use crate::types::transactions::create::TransactionCurrency;

#[derive(Deserialize)]
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
