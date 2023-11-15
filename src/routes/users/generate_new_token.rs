use actix_web::{ web::Data, HttpResponse };
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    types::general::{ ErrorResponse, SuccessResponse },
    utils::emails::send_multipart_email,
};

#[derive(Deserialize, Debug)]
pub struct UserEmail {
    email: String,
}

pub struct SimpleUser {
    id: Uuid,
    email: String,
    display_name: String,
}

#[tracing::instrument(name = "Regenerate token for a user", skip(pool, redis_pool))]
#[actix_web::post("/regenerate-token/")]
pub async fn regenerate_token(
    pool: Data<PgPool>,
    user_email: actix_web::web::Json<UserEmail>,
    redis_pool: Data<deadpool_redis::Pool>
) -> HttpResponse {
    match get_user_who_is_not_active(&pool, &user_email.email).await {
        Ok(visible_user_details) => {
            let mut redis_con = redis_pool
                .get().await
                .map_err(|e| {
                    tracing::event!(target: "redis", tracing::Level::ERROR, "Failed to get redis connection: {:#?}", e);
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "We cannot activate your account at this time. Please try again later.".to_string(),
                    })
                })
                .expect("Redis connection cannot be gotten.");
            send_multipart_email(
                format!("RustDisc - let's Get you verified").to_string(),
                visible_user_details.id,
                visible_user_details.email,
                visible_user_details.display_name,
                "verification_email.html",
                &mut redis_con
            ).await.unwrap();
            HttpResponse::Ok().json(SuccessResponse {
                message: "We have sent you a new verification email.".to_string(),
            });
        }
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "User not found:{:#?}", e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "A user with this e-mail address does not exist. If you registered with this email, ensure you haven't activated it yet. You can check by logging in".to_string(),
            });
        }
    }
    todo!("Regenerate token for a user")
}

#[tracing::instrument(
    name = "Getting a user from db who isn't active",
    , skip(pool, email),fields(user_email = %email)
)]
#[rustfmt::skip]
async fn get_user_who_is_not_active(pool: &PgPool, email: &str) -> Result<SimpleUser, sqlx::Error> {
    match
        sqlx
            ::query_as!(
                SimpleUser,
                r#"
    SELECT id, email, display_name
    FROM users
    WHERE email = $1 AND is_active = false
    "#,
                email
            )
            .fetch_one(pool).await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            tracing::event!(target: "sqlx", tracing::Level::ERROR, "Failed to get user from db: {:#?}", e);
            Err(e)
        }
    }
}
