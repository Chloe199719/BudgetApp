use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use BudgetApp::types::{categories::Category, UserVisible};

use crate::{categories::create_category_in_db, helpers::spawn_app, users::login::LoginUser};
#[derive(Debug, Deserialize, Serialize)]
pub struct EditCategory {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[sqlx::test]
async fn test_edit_category_one_field_name_success(pool: PgPool) {
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

    let edit_category_body = EditCategory {
        name: Some("New Name".to_string()),
        description: None,
    };

    let edit_category_response = app
        .api_client
        .put(&format!(
            "{}/categories/edit/{}",
            app.address, category.category_id
        ))
        .json(&edit_category_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(edit_category_response.status().is_success());

    let edit_category_response_body = edit_category_response
        .json::<Category>()
        .await
        .expect("Failed to parse edit category response");

    assert_eq!(
        edit_category_response_body.category_name,
        edit_category_body.name.unwrap()
    );
}

#[sqlx::test]
async fn test_edit_category_one_field_description_success(pool: PgPool) {
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

    let edit_category_body = EditCategory {
        name: None,
        description: Some("New Description".to_string()),
    };

    let edit_category_response = app
        .api_client
        .put(&format!(
            "{}/categories/edit/{}",
            app.address, category.category_id
        ))
        .json(&edit_category_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(edit_category_response.status().is_success());

    let edit_category_response_body = edit_category_response
        .json::<Category>()
        .await
        .expect("Failed to parse edit category response");

    assert_eq!(
        edit_category_response_body.description,
        edit_category_body.description.unwrap()
    );
}

#[sqlx::test]
async fn test_edit_category_one_field_both_success(pool: PgPool) {
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

    let edit_category_body = EditCategory {
        name: Some("New Name".to_string()),
        description: Some("New Description".to_string()),
    };

    let edit_category_response = app
        .api_client
        .put(&format!(
            "{}/categories/edit/{}",
            app.address, category.category_id
        ))
        .json(&edit_category_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(edit_category_response.status().is_success());

    let edit_category_response_body = edit_category_response
        .json::<Category>()
        .await
        .expect("Failed to parse edit category response");

    assert_eq!(
        edit_category_response_body.description,
        edit_category_body.description.unwrap()
    );
    assert_eq!(
        edit_category_response_body.category_name,
        edit_category_body.name.unwrap()
    );
}
#[sqlx::test]
async fn test_edit_category_one_no_fields_error(pool: PgPool) {
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

    let edit_category_body = EditCategory {
        name: None,
        description: None,
    };

    let edit_category_response = app
        .api_client
        .put(&format!(
            "{}/categories/edit/{}",
            app.address, category.category_id
        ))
        .json(&edit_category_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(edit_category_response.status().is_client_error());
}
