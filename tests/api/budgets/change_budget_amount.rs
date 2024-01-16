use budget_app::types::{categories::Category, UserVisible};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeBudgetAmount {
    pub amount: f64,
}

use crate::{budgets::create_budget::CreateBudget, helpers::spawn_app, users::login::LoginUser};
#[sqlx::test]
async fn test_change_budget_amount(pool: PgPool) {
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

    //Act - Part 2 - Create budget

    let create_budget_body = CreateBudget {
        amount: 100.0,
        start_date: chrono::Utc::now(),
        end_date: chrono::Utc::now() + chrono::Duration::days(30),
        recurring: true,
    };

    let create_budget_response = app
        .api_client
        .post(&format!("{}/budgets/create/{}", app.address, 1))
        .json(&create_budget_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(create_budget_response.status().is_success());

    //Act - Part 3 - Change budget amount
    let change_budget_amount_body = ChangeBudgetAmount { amount: 200.0 };

    let change_budget_amount_response = app
        .api_client
        .patch(&format!("{}/budgets/change_amount/{}", app.address, 1))
        .json(&change_budget_amount_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(change_budget_amount_response.status().is_success());

    let get_category_response = app
        .api_client
        .get(&format!("{}/categories/get/{}", app.address, 1))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(get_category_response.status().is_success());

    let get_category_response_body = get_category_response
        .json::<Category>()
        .await
        .expect("Failed to parse get category response");

    assert_eq!(
        change_budget_amount_body.amount,
        get_category_response_body.amount.unwrap()
    );

    assert_eq!(
        create_budget_body.start_date.timestamp_micros(),
        get_category_response_body
            .start_date
            .unwrap()
            .timestamp_micros()
    );
    assert_eq!(
        create_budget_body.end_date.timestamp_micros(),
        get_category_response_body
            .end_date
            .unwrap()
            .timestamp_micros()
    );
    assert_eq!(
        create_budget_body.recurring,
        get_category_response_body.recurring.unwrap()
    );
    assert_eq!(login_response_body.id, get_category_response_body.user_id);
}

#[sqlx::test]
async fn test_change_budget_amount_error(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };
    let login_response = app.post_login(&login_body).await;
    assert!(login_response.status().is_success());

    //Act - Part 2 - Create budget

    let create_budget_body = CreateBudget {
        amount: 100.0,
        start_date: chrono::Utc::now(),
        end_date: chrono::Utc::now() + chrono::Duration::days(30),
        recurring: true,
    };

    let create_budget_response = app
        .api_client
        .post(&format!("{}/budgets/create/{}", app.address, 1))
        .json(&create_budget_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(create_budget_response.status().is_success());

    //Act - Part 3 - Change budget amount
    let change_budget_amount_body = ChangeBudgetAmount { amount: 200.0 };

    let change_budget_amount_response = app
        .api_client
        .patch(&format!("{}/budgets/change_amount/{}", app.address, 100))
        .json(&change_budget_amount_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(change_budget_amount_response.status().is_client_error());
}
