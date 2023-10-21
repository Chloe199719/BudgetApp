use actix_web::{ get, web::{ Query, Data }, HttpResponse, http::header };
use deadpool_redis::Pool;
use serde::Deserialize;
use sqlx::postgres;

use crate::{
    settings::get_settings,
    types::general::{ ErrorResponse, SuccessResponse },
    utils::auth::tokens::verify_confirmation_token_pasetor,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Parameters {
    token: String,
}

#[tracing::instrument(name = "Activating a new user", skip(pool, parameters, redis_pool))]
#[get("/register/confirm")]
pub async fn config(
    parameters: Query<Parameters>,
    pool: Data<postgres::PgPool>,
    redis_pool: Data<Pool>
) -> HttpResponse {
    let settings = get_settings().expect("Failed to read settings.");
    let mut redis_con = redis_pool
        .get().await
        .map_err(|e| {
            tracing::event!(target: "Discord Backend" , tracing::Level::ERROR, "Failed to get redis connection: {:#?}", e);
            HttpResponse::SeeOther()
                .insert_header((header::LOCATION, format!("{}/auth/error", settings.frontend_url)))
                .json(ErrorResponse {
                    error: "We cannot activate your account at the moment. Please try again later.".to_string(),
                })
        })
        .expect("Redis connection cannot be gotten.");

    let confirmation_token = match
        verify_confirmation_token_pasetor(parameters.token.clone(), &mut redis_con, None).await
    {
        Ok(token) => token,
        Err(e) => {
            tracing::event!(target: "Discord Backend" , tracing::Level::ERROR, "{:#?}", e);
            return HttpResponse::SeeOther()
                .insert_header((
                    header::LOCATION,
                    format!("{}/auth/regenerate-token", settings.frontend_url),
                ))
                .json(ErrorResponse {
                    error: "It appears that your confirmation token has expired or previously used. Kindly generate a new token".to_string(),
                });
        }
    };

    match activate_new_user(&pool, confirmation_token.user_id).await {
        Ok(_) => {
            tracing::event!(target: "Discord Backend" , tracing::Level::INFO, "User successfully activated.");
            HttpResponse::SeeOther()
                .insert_header((
                    header::LOCATION,
                    format!("{}/auth/confirmed", settings.frontend_url),
                ))
                .json(SuccessResponse {
                    message: "Your account has been activated successfully!!! You can now login to your account".to_string(),
                })
        }
        Err(e) => {
            tracing::event!(target: "Discord Backend" , tracing::Level::ERROR, "Cannot activate account : {}", e);
            HttpResponse::SeeOther()
                .insert_header((
                    header::LOCATION,
                    format!("{}/auth/error?reason={e}", settings.frontend_url),
                ))
                .json(ErrorResponse {
                    error: "We cannot activate your account at the moment. Please try again later.".to_string(),
                })
        }
    }
}

#[rustfmt::skip]
#[tracing::instrument(name = "Mark a user active", skip(pool),fields (new_user_user_id = %user_id))]
pub async fn activate_new_user(
    pool: &postgres::PgPool,
    user_id: uuid::Uuid
) -> Result<(), sqlx::Error> {
    match sqlx::query!(
                r#"
            UPDATE users
            SET is_active = true
            WHERE id = $1
        "#,
                user_id
            )
            .execute(pool).await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to execute query: {:#?}", e);
             Err(e)
        }
    }
}
