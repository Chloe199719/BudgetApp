use sqlx::{ Transaction, Postgres };

use crate::utils::constant::BACK_END_TARGET;

#[rustfmt::skip]

#[tracing::instrument(name = "Check if category exists", skip(transaction))]
pub async fn check_category_exists (
    transaction: &mut Transaction<'_, Postgres>,
    category_id: i32,
    user_id: uuid::Uuid
) -> Result<bool, sqlx::Error> {
    match
        sqlx::query!(
                r#"
                    SELECT EXISTS(
                        SELECT 1 FROM categories
                        WHERE category_id = $1 AND user_id = $2
                    )
            "#,
                category_id,
                user_id
            )
            .fetch_one(transaction.as_mut()).await
    {
        Ok(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::INFO, "Successfully checked if category exists");
            Ok(e.exists.unwrap_or(false))
        },
        Err(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists in DB: {:#?}", e);
            Err(e)
        }
    }
}
