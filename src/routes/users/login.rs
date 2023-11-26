use actix_session::Session;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    types::{
        general::{ErrorResponse, USER_EMAIL_KEY, USER_ID_KEY},
        UserVisible,
    },
    utils::{auth::password::verify_password, users::get_active_user_from_db},
};

#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[tracing::instrument(name = "Logging a user in", skip( pool, user, session), fields(user_email = %user.email))]
#[actix_web::post("/login/")]
pub async fn login_user(
    pool: Data<PgPool>,
    user: Json<LoginUser>,
    session: Session,
) -> HttpResponse {
    let password = user.password.clone();
    match get_active_user_from_db(Some(&pool), None, None, Some(&user.email)).await {
        Ok(logged_in_user) => match tokio::task::spawn_blocking(move || {
            return verify_password(logged_in_user.password.as_ref(), password.as_bytes());
        })
        .await
        .expect("Unable to unwrap JoinError.")
        {
            Ok(_) => {
                tracing::event!(target:"Discord Backend", tracing::Level::INFO, "User successfully logged in.");
                session.renew();
                session
                    .insert(USER_ID_KEY, logged_in_user.id)
                    .expect("`user_id` cannot be inserted into session.");
                session
                    .insert(USER_EMAIL_KEY, &logged_in_user.email)
                    .expect("`user_email` cannot be inserted into session.");
                HttpResponse::Ok().json(UserVisible {
                    id: logged_in_user.id,
                    email: logged_in_user.email,
                    display_name: logged_in_user.display_name,
                    unique_name: logged_in_user.unique_name,
                    is_active: logged_in_user.is_active,
                    is_staff: logged_in_user.is_staff,
                    is_superuser: logged_in_user.is_superuser,
                    thumbnail: logged_in_user.thumbnail,
                    data_joined: logged_in_user.data_joined,
                    profile: logged_in_user.profile,
                })
            }
            Err(e) => {
                tracing::event!(target: "argon2",tracing::Level::ERROR, "Failed to authenticate user: {:#?}", e);
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Invalid email or password.".to_string(),
                })
            }
        },
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "Failed to get user from DB: {:#?}", e);
            HttpResponse::NotFound().json(ErrorResponse {
                error: "User with those details doesn't exist".to_string(),
            })
        }
    }
}
