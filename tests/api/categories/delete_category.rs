use discord_backend::types::{
    general::{ErrorResponse, SuccessResponse},
    UserVisible,
};
use sqlx::PgPool;

use crate::{categories::create_category_in_db, helpers::spawn_app, users::login::LoginUser};

#[sqlx::test]
async fn test_delete_category_success(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_success());

    let login_response_body = login_response
        .json::<UserVisible>()
        .await
        .expect("Failed to parse login response");

    //Act - Part 2 - Create category
    let category = create_category_in_db(&pool, login_response_body.id)
        .await
        .expect("Failed to create category");
    //Act - Part 3 - Delete category
    let delete_category_response = app
        .api_client
        .delete(&format!(
            "{}/categories/delete/{}",
            app.address, category.category_id
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(delete_category_response.status().is_success());

    let delete_category_response_body = delete_category_response
        .json::<SuccessResponse>()
        .await
        .expect("Failed to parse delete category response");

    assert_eq!(
        delete_category_response_body.message,
        "Successfully deleted category"
    );
}

#[sqlx::test]
async fn test_delete_category_error_not_logged_in(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    let delete_category_response = app
        .api_client
        .delete(&format!("{}/categories/delete/{}", app.address, 1))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(delete_category_response.status().as_u16(), 401);

    let delete_category_response_body = delete_category_response
        .json::<ErrorResponse>()
        .await
        .expect("Failed to parse delete category response");

    assert_eq!(
        delete_category_response_body.error,
        "You are not logged in. Kindly ensure you are logged in and try again"
    );
}

#[sqlx::test]
async fn test_delete_category_error_category_dosent_exist(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_success());

    //Act - Part 2 - Delete category
    let delete_category_response = app
        .api_client
        .delete(&format!("{}/categories/delete/{}", app.address, 99))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(delete_category_response.status().is_client_error());

    let delete_category_response_body = delete_category_response
        .json::<ErrorResponse>()
        .await
        .expect("Failed to parse delete category response");

    assert_eq!(
        delete_category_response_body.error,
        "Category does not exist"
    );
}
