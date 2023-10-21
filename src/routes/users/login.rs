use actix_session::Session;
use actix_web::{ web::{ Data, Json }, HttpResponse };
use serde::{ Deserialize, Serialize };
use sqlx::PgPool;

use crate::{
    types::{ User, general::{ USER_ID_KEY, USER_EMAIL_KEY, ErrorResponse }, UserVisible },
    utils::auth::password::verify_password,
};

#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    email: String,
    password: String,
}
#[rustfmt::skip]

#[tracing::instrument(name = "Logging a user in", skip( pool, user, session), fields(user_email = %user.email))]
#[actix_web::post("/login/")]
pub async fn login_user(
    pool: Data<PgPool>,
    user: Json<LoginUser>,
    session: Session
) -> HttpResponse {
    let password = user.password.clone();
    match get_user_who_is_active(&pool, &user.email).await {
        Ok(logged_in_user) =>
         
            match tokio::task::spawn_blocking( move|| {
              return   verify_password(logged_in_user.password.as_ref(), password.as_bytes())
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
#[rustfmt::skip]
#[tracing::instrument(name = "Getting a user from DB.", skip(pool,email), fields(user_email = %email))]
pub async fn get_user_who_is_active(pool: &PgPool, email: &str ) -> Result<User, sqlx::Error>{
    match sqlx::query!(
        r#"
        SELECT * FROM users WHERE email = $1 AND is_active = TRUE
        "#,
        email
    ).fetch_one(pool).await {
        Ok(user) => Ok(User {
            id: user.id,
            email: user.email,
            password: user.password,
            display_name: user.display_name,
            unique_name: user.unique_name,
            is_active: true,
            is_staff: user.is_staff,
            is_superuser: user.is_superuser,
            thumbnail: user.thumbnail,
            data_joined: user.data_joined,

        }),
        Err(e) => {
            tracing::event!(target: "sqlx", tracing::Level::ERROR, "Failed to get user from DB: {:#?}", e);
            Err(e)
        }
    }
}
