use discord_backend::{
    types::{ UserVisible, categories::Category, general::ErrorResponse },
    routes::categories::create_category::CreateCategory,
};
use fake::faker::name::en::Name;
use sqlx::PgPool;
use fake::Fake;
use crate::{ helpers::spawn_app, users::login::LoginUser };
// use fake::faker::lorem::en::Paragraph;
use fake::faker::lorem::en::Sentence;
#[sqlx::test]
async fn test_create_category_success(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_success());

    let login_response_body = login_response
        .json::<UserVisible>().await
        .expect("Failed to parse login response");

    //Act - Part 2 - Create category
    let create_category_body = CreateCategory {
        name: Name().fake(),
        description: Sentence(1..2).fake(),
    };

    let create_category_response = app.api_client
        .post(&format!("{}/categories/create", app.address))
        .json(&create_category_body)
        .send().await
        .expect("Failed to execute request.");

    assert!(create_category_response.status().is_success());

    let create_category_response_body = create_category_response
        .json::<Category>().await
        .expect("Failed to parse create category response");

    assert_eq!(create_category_response_body.category_name, create_category_body.name);
    assert_eq!(create_category_response_body.description, create_category_body.description);
    assert_eq!(create_category_response_body.user_id, login_response_body.id);
}

#[sqlx::test]
async fn test_create_category_error_not_logged_in(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    let create_category_body = CreateCategory {
        name: Name().fake(),
        description: Sentence(1..2).fake(),
    };

    let create_category_response = app.api_client
        .post(&format!("{}/categories/create", app.address))
        .json(&create_category_body)
        .send().await
        .expect("Failed to execute request.");

    assert!(create_category_response.status().is_client_error());
    let create_category_response_body = create_category_response
        .json::<ErrorResponse>().await
        .expect("Failed to parse create category response");
    assert_eq!(
        create_category_response_body.error,
        "You are not logged in. Kindly ensure you are logged in and try again"
    );
}
#[sqlx::test]
async fn test_create_category_error_wrong_fields(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_success());

    let login_response_body = login_response
        .json::<UserVisible>().await
        .expect("Failed to parse login response");

    //Act - Part 2 - Create category

    let create_category_response = app.api_client
        .post(&format!("{}/categories/create", app.address))
        .send().await
        .expect("Failed to execute request.");

    assert!(create_category_response.status().is_client_error());
}
