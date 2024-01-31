use actix_web::{web::Data, HttpResponse};
use sqlx::PgPool;

use crate::{
    types::{general::ErrorResponse, UserVisible},
    utils::{constant::BACK_END_TARGET, users::get_active_user_from_db},
};

use super::logout::session_user_id;

#[tracing::instrument(
    name = "Accessing retrieving current user endpoint.",
    skip(pool, session)
)]
#[actix_web::get("/current-user")]
pub async fn get_current_user(pool: Data<PgPool>, session: actix_session::Session) -> HttpResponse {
    match session_user_id(&session).await {
        Ok(id) => match get_active_user_from_db(Some(&pool), None, Some(id), None).await {
            Ok(user) => {
                // session.renew();
                tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "User retrieved from the DB.");
                HttpResponse::Ok().json(UserVisible {
                    id: user.id,
                    email: user.email,
                    display_name: user.display_name,
                    unique_name: user.unique_name,
                    is_active: user.is_active,
                    is_staff: user.is_staff,
                    is_superuser: user.is_superuser,
                    thumbnail: user.thumbnail,
                    data_joined: user.data_joined,
                    profile: user.profile,
                })
            }
            Err(e) => {
                tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "User cannot be retrieved from the DB: {:#?}", e);
                let error_message = ErrorResponse {
                    error: "User was not found".to_string(),
                };
                HttpResponse::NotFound().json(error_message)
            }
        },
        Err(e) => {
            tracing::event!(target: "session",tracing::Level::ERROR, "Failed to get user from session. User unauthorized: {}", e);
            actix_web::HttpResponse::Unauthorized().json(ErrorResponse {
                error: "You are not logged in. Kindly ensure you are logged in and try again"
                    .to_string(),
            })
        }
    }
}
