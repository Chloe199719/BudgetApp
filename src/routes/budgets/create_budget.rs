use actix_web::{
    post,
    web::{Data, Path},
    HttpResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    queries::category::check_category_exists, routes::users::logout::session_user_id,
    types::general::ErrorResponse, utils::constant::BACK_END_TARGET,
};

#[derive(Debug, Deserialize)]
pub struct CreateBudgetPath {
    pub category_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateBudgetPost {
    pub amount: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub recurring: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Budget {
    pub budget_id: i32,
    pub category_id: i32,
    pub user_id: Uuid,
    pub amount: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub recurring: bool,
    pub duration_unix: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[tracing::instrument(name = "Creating a budget", skip(pool, session))]
#[post("/create/{category_id}")]
pub async fn create_budget(
    pool: Data<PgPool>,
    session: actix_session::Session,
    path: Path<CreateBudgetPath>,
    body: actix_web::web::Json<CreateBudgetPost>,
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
    match check_category_exists(&pool, path.category_id, session_uuid).await {
        Ok(false) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: "Category not found".to_string(),
            });
        }
        Ok(true) => {}
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if category exists in DB: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to check if category exists in DB".to_string(),
            });
        }
    }

    match create_budget_db(
        &pool,
        path.category_id,
        session_uuid,
        body.amount,
        body.start_date,
        body.end_date,
        body.recurring,
    )
    .await
    {
        Ok(budget) => {
            tracing::event!(target: BACK_END_TARGET , tracing::Level::INFO, "Budget created successfully: {:?}", budget);
            HttpResponse::Ok().json(budget)
        }
        Err(e) => {
            tracing::event!(target: "sqlx", tracing::Level::ERROR, "Failed to create budget: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to create budget".to_string(),
            })
        }
    }
}

#[tracing::instrument(name = "Creating a budget in Db", skip(pool))]
pub async fn create_budget_db(
    pool: &PgPool,
    category_id: i32,
    user_id: Uuid,
    amount: f64,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    recurring: bool,
) -> Result<Budget, sqlx::Error> {
    let duration_unix = end_date.timestamp() - start_date.timestamp();
    let budget = sqlx::query_as!(
        Budget,
        r#"
        INSERT INTO budgets (category_id, user_id, amount, start_date, end_date, recurring, duration_unix)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING budget_id, category_id, user_id, amount, start_date, end_date, recurring, duration_unix, created_at, updated_at
        "#,
        category_id,
        user_id,
        amount,
        start_date,
        end_date,
        recurring,
        duration_unix
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        UPDATE categories
        SET budget_id = $1
        WHERE category_id = $2 AND user_id = $3

        "#,
        budget.budget_id,
        category_id,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(budget)
}
