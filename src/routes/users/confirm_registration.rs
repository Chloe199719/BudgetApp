use actix_web::{ get, web::{ Query, Data }, HttpResponse, http::header };
use deadpool_redis::Pool;
use serde::Deserialize;
use sqlx::postgres;

use crate::{ settings::get_settings, types::general::ErrorResponse };

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
    todo!("Implement this function.")
}
