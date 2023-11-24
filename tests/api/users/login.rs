use discord_backend::types::{ general::ErrorResponse, UserVisible };
use fake::{ faker::{ name::en::NameWithTitle, internet::en::SafeEmail }, Fake };
use serde::{ Serialize, Deserialize };
use sqlx::PgPool;

use crate::helpers::spawn_app;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
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

#[sqlx::test]
async fn test_login_user_success(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_success());

    //Check that there is a cookie present

    let headers = login_response.headers();
    assert!(headers.get("set-cookie").is_some());

    let cookie_str = headers.get("set-cookie").unwrap().to_str().unwrap();
    assert!(cookie_str.contains("sessionid="));

    // Check response

    let response = login_response.json::<UserVisible>().await.expect("Failed to parse response");

    assert_eq!(response.email, app.test_user.email);
    assert!(response.is_active);
    assert_eq!(response.id, response.profile.user_id);
}
