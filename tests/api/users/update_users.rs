use discord_backend::types::general::ErrorResponse;
use reqwest::multipart::Form;
use sqlx::PgPool;

use crate::helpers::spawn_app;

#[sqlx::test]
async fn test_update_user_failure_not_logged_in(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    // multipart form data
    let form = get_multipart_form_data();
    let update_user_response = app.api_client
        .patch(&format!("{}/users/update_user", app.address))
        .multipart(form)
        .send().await
        .expect("Failed to execute request.");

    //Check response
    let response = update_user_response
        .json::<ErrorResponse>().await
        .expect("Failed to deserialize response");

    assert_eq!(
        response.error,
        "You are not logged in. Kindly ensure you are logged in and try again"
    );
}

fn get_multipart_form_data() -> Form {
    Form::new()
        .text("github_link", "https://github.com/Chloe199719")
        .text("about_me", "I am Software Engineer in Test")
        .text("pronouns", "she/her")
        .text("phone_number", "+447123456789")
}
