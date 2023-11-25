use discord_backend::types::{ UserVisible, general::SuccessResponse };
use sqlx::PgPool;

use crate::{ users::login::LoginUser, helpers::spawn_app, categories::create_category_in_db };

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
        .json::<UserVisible>().await
        .expect("Failed to parse login response");

    //Act - Part 2 - Create category
    let category_id = create_category_in_db(&pool, login_response_body.id).await.expect(
        "Failed to create category"
    );
    println!("category_id: {}", category_id);
    //Act - Part 3 - Delete category
    let delete_category_response = app.api_client
        .delete(&format!("{}/categories/delete/{}", app.address, category_id))
        .send().await
        .expect("Failed to execute request.");

    assert!(delete_category_response.status().is_success());

    let delete_category_response_body = delete_category_response
        .json::<SuccessResponse>().await
        .expect("Failed to parse delete category response");

    assert_eq!(delete_category_response_body.message, "Successfully deleted category");
}
