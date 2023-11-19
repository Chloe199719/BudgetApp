use discord_backend::types::{ general::ErrorResponse, UserVisible };
use serde::{ Serialize, Deserialize };
use sqlx::PgPool;

use crate::helpers::spawn_app;

#[derive(Deserialize, Debug, Serialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[sqlx::test]
async fn test_get_current_user_failure(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    // Then get current user
    let get_user_response = app.api_client
        .get(&format!("{}/users/current-user", &app.address))
        .send().await
        .expect("Failed to execute request.");

    // Check response
    let response = get_user_response
        .json::<ErrorResponse>().await
        .expect("Cannot get user response");

    assert_eq!(
        response.error,
        "You are not logged in. Kindly ensure you are logged in and try again"
    );
}

#[sqlx::test]
async fn test_get_current_user_success(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    // First login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };
    let login_response = app.post_login(&login_body).await;
    assert!(login_response.status().is_success());

    // Then get current user
    let get_user_response = app.api_client
        .get(&format!("{}/users/current-user", &app.address))
        .send().await
        .expect("Failed to execute request.");

    // Check response
    let response = get_user_response.json::<UserVisible>().await.expect("Cannot get user response");

    assert_eq!(response.email, app.test_user.email);
    assert!(response.is_active);
    assert_eq!(response.id, response.profile.user_id);
}
