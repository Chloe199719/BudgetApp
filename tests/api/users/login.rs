use discord_backend::types::general::ErrorResponse;
use fake::{ faker::{ name::en::NameWithTitle, internet::en::SafeEmail }, Fake };
use serde::{ Serialize, Deserialize };
use sqlx::PgPool;

use crate::helpers::spawn_app;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[sqlx::test]
async fn test_login_user_failure_bad_request(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: NameWithTitle().fake(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_client_error());

    let error_response = login_response
        .json::<ErrorResponse>().await
        .expect("Failed to parse error response");

    assert_eq!(error_response.error, "Invalid email or password.");
}

#[sqlx::test]
async fn test_login_user_failure_not_found(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Act - Part 1 - Login

    let login_body = LoginUser {
        email: SafeEmail().fake(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_client_error());

    let error_response = login_response
        .json::<ErrorResponse>().await
        .expect("Failed to parse error response");

    assert_eq!(error_response.error, "User with those details doesn't exist");
}
