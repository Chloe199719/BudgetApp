use sqlx::PgPool;

#[tracing::instrument(name = "Check Budget exists in DB", skip(pool))]
pub async fn check_budget_exists_db(
    pool: &PgPool,
    budget_id: i32,
    user_id: uuid::Uuid,
) -> Result<bool, sqlx::Error> {
    match sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM budgets
            WHERE budget_id = $1 AND user_id = $2
        ) AS "exists!";
        "#,
        budget_id,
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(result) => Ok(result.exists),
        Err(e) => Err(e),
    }
}

#[tracing::instrument(name = "Delete Budget in DB", skip(pool))]
pub async fn delete_budget_db(
    pool: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    budget_id: i32,
    user_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM budgets
        WHERE budget_id = $1 AND user_id = $2
        RETURNING budget_id, category_id;
        "#,
        budget_id,
        user_id
    )
    .fetch_one(pool.as_mut())
    .await?;
    sqlx::query!(
        r#"
        UPDATE categories
        SET budget_id = NULL
        WHERE budget_id = $1 AND user_id = $2;
        "#,
        budget_id,
        user_id
    )
    .execute(pool.as_mut())
    .await?;
    Ok(())
}

#[tracing::instrument(name = "Change Budget Amount in DB", skip(pool))]
pub async fn change_budget_amount_db(
    pool: &PgPool,
    budget_id: i32,
    user_id: uuid::Uuid,
    amount: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE budgets
        SET amount = $1
        WHERE budget_id = $2 AND user_id = $3;
        "#,
        amount,
        budget_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tracing::instrument(name = "Change Budget Date in DB", skip(pool))]
pub async fn change_budget_date_db(
    pool: &PgPool,
    budget_id: i32,
    user_id: uuid::Uuid,
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
) -> Result<(), sqlx::Error> {
    let duration_unix = end_date.timestamp() - start_date.timestamp();
    sqlx::query!(
        r#"
        UPDATE budgets
        SET start_date = $1, end_date = $2, duration_unix = $3
        WHERE budget_id = $4 AND user_id = $5;
        "#,
        start_date,
        end_date,
        duration_unix,
        budget_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tracing::instrument(name = "Change Budget Recursing in DB", skip(pool))]
pub async fn change_budget_recursing_db(
    pool: &PgPool,
    budget_id: i32,
    user_id: uuid::Uuid,
    recurring: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE budgets
        SET recurring = $1
        WHERE budget_id = $2 AND user_id = $3;
        "#,
        recurring,
        budget_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
