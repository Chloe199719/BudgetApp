use discord_backend::types::general::{ErrorResponse, SuccessResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::helpers::spawn_app;

#[derive(Debug, Deserialize, Serialize)]
struct LoginUser {
    email: String,
    password: String,
}
#[sqlx::test]
async fn tesst_logout_failure(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Test Logout User
    let logout_response = app
        .api_client
        .post(&format!("{}/users/logout/", app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(logout_response.status().is_client_error());
    //Check logout_response

    let response = logout_response
        .json::<ErrorResponse>()
        .await
        .expect("Failed to deserialize response");

    assert_eq!(response.error, "Failed to log user out.");
}

#[sqlx::test]
async fn test_logout_sucess(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //First login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;
    assert!(login_response.status().is_success());

    //Check that there is cookie present
    let headers = login_response.headers();
    assert!(headers.get("set-cookie").is_some());
    let cookie_str = headers.get("set-cookie").unwrap().to_str().unwrap();
    assert!(cookie_str.contains("sessionid="));

    //Test Logout users
    let logout_response = app
        .api_client
        .post(&format!("{}/users/logout/", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Check response

    let response = logout_response
        .json::<SuccessResponse>()
        .await
        .expect("Failed to deserialize response");

    assert_eq!(response.message, "User successfully logged out.");
}
