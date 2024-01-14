use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};

use serde::Deserialize;

use crate::{
    queries::budget::{change_budget_amount_db, check_budget_exists_db},
    routes::users::logout::session_user_id,
    types::general::{ErrorResponse, SuccessResponse},
    utils::constant::BACK_END_TARGET,
};

#[derive(Debug, Deserialize)]
pub struct ChangeBudgetBody {
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct ChangeBudgetPath {
    pub budget_id: i32,
}

#[tracing::instrument(name = "Change Budget Amount", skip(pool, session))]
#[actix_web::put("/change_amount/{budget_id}")]
pub async fn change_budget_amount(
    pool: Data<sqlx::PgPool>,
    path: Path<ChangeBudgetPath>,
    body: Json<ChangeBudgetBody>,
    session: actix_session::Session,
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
    match check_budget_exists_db(&pool, path.budget_id, session_uuid).await {
        Ok(false) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: "Budget not found".to_string(),
            });
        }
        Ok(true) => {}
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to check if budget exists in DB: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to check if budget exists in DB".to_string(),
            });
        }
    }

    match change_budget_amount_db(&pool, path.budget_id, session_uuid, body.amount).await {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            message: "Budget amount changed successfully".to_string(),
        }),
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to change budget amount in DB: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to change budget amount in DB".to_string(),
            })
        }
    }
}
