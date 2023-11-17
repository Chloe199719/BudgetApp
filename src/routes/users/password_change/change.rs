use actix_web::{post, web::{Data, Json}, HttpResponse, http::header::LOCATION};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{settings::get_settings, utils::{auth::{tokens::verify_confirmation_token_pasetor, password::hash}, constant::BACK_END_TARGET}, types::general::SuccessResponse};

#[derive(Deserialize, Debug)]
pub struct NewPassword {
    token: String,
    password: String,
}


#[tracing::instrument(name = "Changing user password", skip(pool, new_password,reds_pool))]
#[post("/change-user-password")]
pub async fn change_user_password(
    pool: Data<PgPool>,
    new_password: Json<NewPassword>,
    reds_pool: Data<deadpool_redis::Pool>,
)-> HttpResponse{
    let settings = get_settings().expect("Failed to read settings");
    let mut redis_con = reds_pool.get().await.map_err(|e|{
        tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "{}",e);
        HttpResponse::SeeOther().insert_header((LOCATION, format!("{}/auth/error?reason=We cannot activate your account at the moment. Please try again later", settings.frontend_url))).finish()
    }).expect("Failed to get redis connection");
    let confirmation_token = match  verify_confirmation_token_pasetor(new_password.0.token, &mut redis_con, Some(true)).await 
    {
        Ok(token) => token,
        Err(e) => {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "{:#?}",e);
            return HttpResponse::SeeOther().insert_header((LOCATION, format!("{}/auth/error?reason=It appears that your password request token has expired or previously used", settings.frontend_url))).finish();
        }
    };
    let new_user_password = hash(new_password.0.password.as_bytes()).await;

    match update_user_password_in_db(&pool,&new_user_password, confirmation_token.user_id).await {
        Ok(_) =>{
            tracing::event!(target: BACK_END_TARGET, tracing::Level::INFO, "User password has been updated successfully");
            HttpResponse::Ok().json(SuccessResponse {
            message:"Password has been changed successfully".to_string(),
            })
        }
        Err(e)=> {
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "Failed to update user password in the DB {:#?}",e);
            HttpResponse::BadRequest().json(crate::types::general::ErrorResponse {
                error: "Failed to update user password".to_string(),
            })
        }
    }
}

#[tracing::instrument(name = "Updating user password in the dB", skip(pool, new_password))]
async fn update_user_password_in_db(
    pool:&PgPool,
    new_password: &String,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    match sqlx::query!(
        r#"
        UPDATE users
        SET password = $1
        WHERE id = $2
        "#,
        new_password,
        user_id
    )
    .execute(pool)
    .await
    {
        Ok(r) =>{
            tracing::event!(target:"SQLX", tracing::Level::INFO, "User password has been updated successfully in the DB {:#?}", r);
            Ok(true)
        }
        Err(e) => {
            tracing::event!(target:"SQLX", tracing::Level::ERROR, "Failed to update user password in the DB {:#?}", e);
            Err(e)
        }
    }
}
