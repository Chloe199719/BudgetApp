use budget_app::types::{categories::Category, UserVisible};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    categories::{create_category_in_db, TEST_CATEGORY_DESCRIPTION, TEST_CATEGORY_NAME},
    helpers::spawn_app,
    users::login::LoginUser,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct EditCategory {
    id: i32,
}

#[sqlx::test]
async fn test_get_category_by_id_success(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;
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

    let category = create_category_in_db(&pool, login_response_body.id)
        .await
        .expect("Failed to create category");

    let get_category_response = app
        .api_client
        .get(&format!("{}/categories/get", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(get_category_response.status().is_success());

    let get_category_response_body = get_category_response
        .json::<Vec<Category>>()
        .await
        .expect("Failed to parse get category response");

    // Check if the last category in the vector is the one we just created
    assert_eq!(
        get_category_response_body.last().unwrap().category_name,
        category.category_name
    );
    assert_eq!(
        get_category_response_body.last().unwrap().description,
        category.description
    );
    assert_eq!(
        get_category_response_body.last().unwrap().user_id,
        category.user_id
    );

    // Check if the first category in the vector is the default
    assert_eq!(
        get_category_response_body.first().unwrap().category_name,
        TEST_CATEGORY_NAME
    );
    assert_eq!(
        get_category_response_body.first().unwrap().description,
        TEST_CATEGORY_DESCRIPTION
    );
}

#[sqlx::test]
async fn test_get_category_by_id_default_success(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_success());

    let get_category_response = app
        .api_client
        .get(&format!("{}/categories/get", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(get_category_response.status().is_success());

    let get_category_response_body = get_category_response
        .json::<Vec<Category>>()
        .await
        .expect("Failed to parse get category response");

    println!("{:?}", get_category_response_body);

    assert_eq!(
        get_category_response_body.first().unwrap().category_name,
        TEST_CATEGORY_NAME
    );
    assert_eq!(
        get_category_response_body.first().unwrap().description,
        TEST_CATEGORY_DESCRIPTION
    );
}
