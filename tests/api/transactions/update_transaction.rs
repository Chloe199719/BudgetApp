use budget_app::types::{
    transactions::create::{TransactionCurrency, TransactionType},
    UserVisible,
};
use chrono::Utc;
use fake::{faker::lorem::en::Sentence, Fake};
use reqwest::multipart::Form;
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    helpers::spawn_app,
    transactions::create_a_transaction::CreateTransactionWithCurrencyAndCategory,
    users::login::LoginUser,
};
#[derive(Debug, Clone, Serialize)]
pub struct UpdateTransaction {
    pub amount: Option<f64>,
    pub currency: Option<TransactionCurrency>,
    pub description: Option<String>,
}

#[sqlx::test]
async fn test_update_transaction_success_change_amount(pool: PgPool) {
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

    let update_transaction_body = UpdateTransaction {
        amount: Some(200.0),
        currency: None,
        description: None,
    };

    let response = app
        .api_client
        .patch(&format!(
            "{}/transactions/{}/update",
            app.address, create_transaction_response_body.transaction_id
        ))
        .multipart(Form::new().text(
            "amount",
            update_transaction_body.amount.unwrap().to_string(),
        ))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let response_body = response
        .json::<budget_app::types::transactions::create::TransactionOutcomeWithReceipt>()
        .await
        .expect("Failed to parse response");

    assert_eq!(response_body.amount, 200.0);

    assert_eq!(
        create_transaction_response_body.transaction_type,
        "DEPOSIT".to_string()
    );
    assert_eq!(
        create_transaction_response_body.description,
        create_transaction_body.description
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
}
#[sqlx::test]
async fn test_update_transaction_success_change_currency(pool: PgPool) {
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

    let update_transaction_body = UpdateTransaction {
        amount: None,
        currency: Some(TransactionCurrency::EUR),
        description: None,
    };

    let response = app
        .api_client
        .patch(&format!(
            "{}/transactions/{}/update",
            app.address, create_transaction_response_body.transaction_id
        ))
        .multipart(
            Form::new().text(
                "currency",
                update_transaction_body
                    .currency
                    .clone()
                    .unwrap()
                    .to_string(),
            ),
        )
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let response_body = response
        .json::<budget_app::types::transactions::create::TransactionOutcomeWithReceipt>()
        .await
        .expect("Failed to parse response");

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
        response_body.currency.to_string(),
        update_transaction_body.currency.unwrap().to_string()
    );
    assert_eq!(
        create_transaction_response_body.category_id,
        create_transaction_body.category_id
    );
}
