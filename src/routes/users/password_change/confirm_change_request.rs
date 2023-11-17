use actix_web::{get, HttpResponse, http::header::LOCATION, web::{Query, Data}};
use serde::Deserialize;

use crate::{settings::get_settings, utils::{auth::tokens::{verify_confirmation_token_pasetor, issue_confirmation_token_pasetors}, constant::BACK_END_TARGET}, types::general::ErrorResponse};

#[derive(Deserialize, Debug)]
pub struct Parameters {
    token :String,
}

#[tracing::instrument(name = "Confirming change password token" , skip(params,redis_pool))]
#[get("/confirm/changepassword")]
pub async fn confirm_change_password_token(
    params: Query<Parameters>,
    redis_pool: Data<deadpool_redis::Pool>,
) -> HttpResponse {
    let settings = get_settings().expect("Failed to read settings");
    let mut redis_con = redis_pool.get().await.map_err(|e|{
        tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR ,"Failed to get redis connection: {}",e);
        
        HttpResponse::SeeOther().insert_header((LOCATION, format!("{}/auth/error?reason=Something unexpected happened. Please try again later", settings.frontend_url))).finish()
    }).expect("Failed to get redis connection");
    let confirmation_token = match verify_confirmation_token_pasetor(params.token.clone(), &mut redis_con, None)
        .await
        {
            Ok(token) => token,
            Err(e) => {
                tracing::event!(target:BACK_END_TARGET, tracing::Level::ERROR, "{:#?}",e);
                return HttpResponse::SeeOther().insert_header((LOCATION, format!("{}/auth/error?reason=It appears that your password request token has expired or previously used", settings.frontend_url))).finish();
            }
        };
    let issued_token = match issue_confirmation_token_pasetors(confirmation_token.user_id, &mut redis_con, Some(true)).await
    {
        Ok(token) => token,
        Err(e) =>{
            tracing::event!(target: BACK_END_TARGET, tracing::Level::ERROR, "{:#?}",e);
            return HttpResponse::SeeOther().insert_header((LOCATION,format!("{}/auth/error?reason={}",settings.frontend_url,e))).json(ErrorResponse {
                error: format!("{}",e),
            })
        }
    };
    HttpResponse::SeeOther().insert_header((LOCATION, format!("{}/auth/password/change-password?token={}", settings.frontend_url, issued_token))).finish()

}

