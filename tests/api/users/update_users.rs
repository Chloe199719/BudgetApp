use budget_app::types::{general::ErrorResponse, UserVisible};
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::helpers::spawn_app;
#[derive(Serialize, Deserialize, Debug)]
struct LoginUser {
    email: String,
    password: String,
}
const GITHUB_LINK: &str = "https://github.com/Chloe199719";
const ABOUT_ME: &str = "I am Software Engineer in Test";
const PRONOUNS: &str = "she/her";
const PHONE_NUMBER: &str = "+447123456789";

#[sqlx::test]
async fn test_update_user_failure_not_logged_in(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    // multipart form data
    let form = get_multipart_form_data();
    let update_user_response = app
        .api_client
        .patch(&format!("{}/users/update_user", app.address))
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    //Check response
    let response = update_user_response
        .json::<ErrorResponse>()
        .await
        .expect("Failed to deserialize response");

    assert_eq!(
        response.error,
        "You are not logged in. Kindly ensure you are logged in and try again"
    );
}

#[sqlx::test]
async fn test_update_user_success(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    // First login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;
    assert!(login_response.status().is_success());

    // Check that there is cookie present

    let headers = login_response.headers();
    assert!(headers.get("set-cookie").is_some());

    let cookie_str = headers.get("set-cookie").unwrap().to_str().unwrap();

    assert!(cookie_str.contains("sessionid="));

    // multipart form
    let form = get_multipart_form_data();

    let update_user_response = app
        .api_client
        .patch(&format!("{}/users/update_user", &app.address))
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    // Check response

    let response = update_user_response
        .json::<UserVisible>()
        .await
        .expect("Cannot get user response");

    assert_eq!(response.email, app.test_user.email);
    assert!(response.is_active);
    assert_eq!(response.id, response.profile.user_id);
    assert_eq!(response.profile.github_link, Some(GITHUB_LINK.to_string()));
    assert_eq!(response.profile.about_me, Some(ABOUT_ME.to_string()));
    assert_eq!(response.profile.pronouns, Some(PRONOUNS.to_string()));
    assert_eq!(
        response.profile.phone_number,
        Some(PHONE_NUMBER.to_string())
    );
}

fn get_multipart_form_data() -> Form {
    Form::new()
        .text("github_link", GITHUB_LINK)
        .text("about_me", ABOUT_ME)
        .text("pronouns", PRONOUNS)
        .text("phone_number", PHONE_NUMBER)
}
