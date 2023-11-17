use serde::Deserialize;
use actix_web::{post, web::{Data, Json}, HttpResponse};
use sqlx::PgPool;

use crate::{settings::get_settings, utils::{users::get_active_user_from_db, emails::send_multipart_email, constant::{APP_NAME, BACK_END_TARGET}}, types::general::{ErrorResponse, SuccessResponse}};
#[derive(Deserialize, Debug)]
pub struct UserEmail {
    email: String,
}

#[tracing::instrument(name = "Requesting a password Change", skip(pool,redis_pool))]
#[post("/request-password-change")]
pub async fn request_password_change (
pool: Data<PgPool>,
user_email: Json<UserEmail>,
redis_pool: Data<deadpool_redis::Pool>) -> HttpResponse {
    let settings = get_settings().expect("Failed to read settings");
    match get_active_user_from_db(Some(&pool), None, None, Some(&user_email.email)).await {
        Ok(visible_user_details) => {
            let mut redis_con = redis_pool.get().await.map_err(|e|{
                tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR ,"Failed to get redis connection: {}",e);
                HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Something went wrong. Please try again later".to_string(),
                })
            }).expect("Failed to get redis connection");

            send_multipart_email(format!("{} - Password Reset Instructions",{APP_NAME}), visible_user_details.id,visible_user_details.email, visible_user_details.display_name, "password_reset_email.html", &mut redis_con).await.unwrap();
             HttpResponse::Ok().json(SuccessResponse {
                message: "Password reset instructions sent to your email".to_string(),   
            })
    }
        Err(e) => {
            tracing::event!(target:"SQLX",tracing::Level::ERROR, "Error getting user from db: {}",e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: format!("An active user with this e-mail address does not exist. If you registered with this email, ensure you have activated your account. You can check by logging in. If you have not activated it, visit {}/auth/regenerate-token to regenerate the token that will allow you activate your account.", settings.frontend_url),
            })
        }
             
}
}
