use budget_app::{
    routes::categories::create_category::CreateCategory,
    types::{categories::Category, transactions::create::TransactionType, UserVisible},
};
use chrono::Utc;
use fake::{
    faker::{lorem::en::Sentence, name::en::Name},
    Fake,
};
use reqwest::multipart::Form;
use sqlx::PgPool;

use crate::{
    helpers::spawn_app,
    transactions::create_a_transaction::CreateTransactionWithCurrencyAndCategory,
    users::login::LoginUser,
};

#[sqlx::test]
async fn test_swap_category_success(pool: PgPool) {
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

    //Act - Part 2 - Create transaction

    let create_transaction_body = CreateTransactionWithCurrencyAndCategory {
        transaction_date: Utc::now(),
        transaction_type: TransactionType::DEPOSIT,
        description: Sentence(1..2).fake(),
        amount: 100.0,
        currency: "USD".to_string(),
        category_id: 1,
    };

    let create_transaction_response = app
        .api_client
        .post(&format!("{}/transactions/create", app.address))
        .multipart(
            Form::new()
                .text(
                    "transaction_date",
                    create_transaction_body.transaction_date.to_rfc3339(),
                )
                .text(
                    "transaction_type",
                    create_transaction_body.transaction_type.to_string(),
                )
                .text("description", create_transaction_body.description.clone())
                .text("amount", create_transaction_body.amount.to_string())
                .text("currency", create_transaction_body.currency.clone())
                .text(
                    "category_id",
                    create_transaction_body.category_id.to_string(),
                ),
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(create_transaction_response.status().is_success());

    let create_transaction_response_body = create_transaction_response
        .json::<budget_app::types::transactions::create::TransactionOutcomeWithReceipt>()
        .await
        .expect("Failed to parse create transaction response");

    assert_eq!(
        create_transaction_response_body.transaction_type,
        "DEPOSIT".to_string()
    );
    assert_eq!(
        create_transaction_response_body.description,
        create_transaction_body.description
    );
    assert_eq!(
        create_transaction_response_body.amount,
        create_transaction_body.amount
    );
    assert_eq!(
        create_transaction_response_body.user_id,
        login_response_body.id
    );
    assert_eq!(
        create_transaction_response_body.date.date_naive(),
        create_transaction_body.transaction_date.date_naive()
    );
    assert_eq!(
        create_transaction_response_body.currency.to_string(),
        create_transaction_body.currency.to_string()
    );
    assert_eq!(
        create_transaction_response_body.category_id,
        create_transaction_body.category_id
    );
    let create_category_body = CreateCategory {
        name: Name().fake(),
        description: Sentence(1..2).fake(),
    };

    let create_category_response = app
        .api_client
        .post(&format!("{}/categories/create", app.address))
        .json(&create_category_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(create_category_response.status().is_success());

    let create_category_response_body = create_category_response
        .json::<Category>()
        .await
        .expect("Failed to parse create category response");

    let swap_category_response = app
        .api_client
        .patch(&format!(
            "{}/transactions/swap_category/{}/{}",
            app.address,
            create_transaction_response_body.transaction_id,
            create_category_response_body.category_id
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(swap_category_response.status().is_success());
}
#[sqlx::test]
async fn test_swap_category_error_category_dosent_exist(pool: PgPool) {
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

    //Act - Part 2 - Create transaction

    let create_transaction_body = CreateTransactionWithCurrencyAndCategory {
        transaction_date: Utc::now(),
        transaction_type: TransactionType::DEPOSIT,
        description: Sentence(1..2).fake(),
        amount: 100.0,
        currency: "USD".to_string(),
        category_id: 1,
    };

    let create_transaction_response = app
        .api_client
        .post(&format!("{}/transactions/create", app.address))
        .multipart(
            Form::new()
                .text(
                    "transaction_date",
                    create_transaction_body.transaction_date.to_rfc3339(),
                )
                .text(
                    "transaction_type",
                    create_transaction_body.transaction_type.to_string(),
                )
                .text("description", create_transaction_body.description.clone())
                .text("amount", create_transaction_body.amount.to_string())
                .text("currency", create_transaction_body.currency.clone())
                .text(
                    "category_id",
                    create_transaction_body.category_id.to_string(),
                ),
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(create_transaction_response.status().is_success());

    let create_transaction_response_body = create_transaction_response
        .json::<budget_app::types::transactions::create::TransactionOutcomeWithReceipt>()
        .await
        .expect("Failed to parse create transaction response");

    assert_eq!(
        create_transaction_response_body.transaction_type,
        "DEPOSIT".to_string()
    );
    assert_eq!(
        create_transaction_response_body.description,
        create_transaction_body.description
    );
    assert_eq!(
        create_transaction_response_body.amount,
        create_transaction_body.amount
    );
    assert_eq!(
        create_transaction_response_body.user_id,
        login_response_body.id
    );
    assert_eq!(
        create_transaction_response_body.date.date_naive(),
        create_transaction_body.transaction_date.date_naive()
    );
    assert_eq!(
        create_transaction_response_body.currency.to_string(),
        create_transaction_body.currency.to_string()
    );
    assert_eq!(
        create_transaction_response_body.category_id,
        create_transaction_body.category_id
    );

    let swap_category_response = app
        .api_client
        .patch(&format!(
            "{}/transactions/swap_category/{}/{}",
            app.address, create_transaction_response_body.transaction_id, 100
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(swap_category_response.status().is_client_error());
}
