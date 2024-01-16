use actix_web::{
    patch,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::Deserialize;

use crate::{
    queries::budget::{change_budget_recursing_db, check_budget_exists_db},
    routes::users::logout::session_user_id,
    types::general::{ErrorResponse, SuccessResponse},
    utils::constant::BACK_END_TARGET,
};

#[derive(Debug, Deserialize)]
pub struct ChangeBudgetBody {
    pub recurring: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChangeBudgetPath {
    pub budget_id: i32,
}

#[tracing::instrument(name = "Change Budget Recursing", skip(pool, session))]
#[patch("/change_recurring/{budget_id}")]
pub async fn change_budget_recursing(
    pool: Data<sqlx::PgPool>,
    path: Path<ChangeBudgetPath>,
    body: Json<ChangeBudgetBody>,
    session: actix_session::Session,
) -> HttpResponse {
    let session_uuid = match session_user_id(&session).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "session", tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            return HttpResponse::Unauthorized().json(ErrorResponse {
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

    match change_budget_recursing_db(&pool, path.budget_id, session_uuid, body.recurring).await {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            message: "Budget recursing changed successfully".to_string(),
        }),
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to change budget recursing in DB: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to change budget recursing in DB".to_string(),
            })
        }
    }
}
