use sqlx::PgPool;

use crate::{ types::categories::Category, utils::constant::BACK_END_TARGET };

#[rustfmt::skip]

#[tracing::instrument(name = "Check if category exists", skip(pool))]
pub async fn check_category_exists (
    pool: &PgPool,
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
            .fetch_one(pool).await
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
#[rustfmt::skip]

#[tracing::instrument(name = "Check if category exists", skip(pool))]
pub async fn check_category_exists_return_it (
    pool:  &sqlx::PgPool,
    category_id: i32,
    user_id: uuid::Uuid
) -> Result<Category, sqlx::Error> {
    match
        sqlx::query_as!(
            Category,
                    r#"
                        SELECT * FROM categories
                        WHERE category_id = $1 AND user_id = $2
                    "#,
                category_id,
                user_id
            )
            .fetch_one(pool).await
    {
        Ok(e) => {
            tracing::event!(target:BACK_END_TARGET, tracing::Level::INFO, "Category exits returning it");
            Ok(e)
        },
        Err(e) => {
           
            match e {
                sqlx::Error::RowNotFound => {
                    tracing::event!(target:BACK_END_TARGET, tracing::Level::INFO, "Category does not exist");
                    Err(e)
                
                }
                _ => {
                    tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists in DB: {:#?}", e);
                    Err(e)
                }
            }
        }
    }
}
