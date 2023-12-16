use crate::types::transactions::create::{
    Transaction, TransactionCurrency, TransactionOutcome, TransactionOutcomeWithReceipt,
    TransactionType,
};

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
        &transaction_data.transaction_type as &TransactionType,
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
