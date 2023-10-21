use actix_session::Session;
use actix_web::{ post, HttpResponse };
use uuid::Uuid;

use crate::types::general::{ USER_ID_KEY, SuccessResponse, ErrorResponse };

#[tracing::instrument(name = "Log out user", skip(session))]
#[post("/logout/")]
pub async fn logout_user(session: Session) -> HttpResponse {
    match session_user_id(&session).await {
        Ok(_) => {
            tracing::event!(target: "Discord Backend" , tracing::Level::INFO, "User successfully logged out.");
            session.purge();
            HttpResponse::Ok().json(SuccessResponse {
                message: "User successfully logged out.".to_string(),
            })
        }
        Err(e) => {
            tracing::event!(target: "Discord Backend" , tracing::Level::ERROR, "Failed to log user out: {:#?}", e);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "Failed to log user out.".to_string(),
            })
        }
    }
}

#[tracing::instrument(name = "Get user_id from session", skip(session))]
async fn session_user_id(session: &Session) -> Result<Uuid, String> {
    match session.get(USER_ID_KEY) {
        Ok(user_id) =>
            match user_id {
                Some(user_id) => Ok(user_id),
                None => Err("You are not authenticated.".to_string()),
            }
        Err(e) => Err(format!("Failed to get user_id from session: {:#?}", e)),
    }
}
