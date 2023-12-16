use crate::{helpers::spawn_app, users::login::LoginUser};
use budget_app::types::{transactions::create::TransactionType, UserVisible};
use chrono::Utc;
use fake::faker::lorem::en::Sentence;
use fake::Fake;
use reqwest::multipart::Form;
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateTransaction {
    pub transaction_date: chrono::DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub description: String,
    pub amount: f64,
}

#[sqlx::test]
async fn test_create_transaction_success_default_category_and_currency(pool: PgPool) {
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

    let create_transaction_body = CreateTransaction {
        transaction_date: Utc::now(),
        transaction_type: TransactionType::DEPOSIT,
        description: Sentence(1..2).fake(),
        amount: 100.0,
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
                .text("amount", create_transaction_body.amount.to_string()),
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
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateTransactionWithCurrency {
    pub transaction_date: chrono::DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub description: String,
    pub amount: f64,
    pub currency: String,
}
#[sqlx::test]
async fn test_create_transaction_success_default_category(pool: PgPool) {
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

    let create_transaction_body = CreateTransactionWithCurrency {
        transaction_date: Utc::now(),
        transaction_type: TransactionType::DEPOSIT,
        description: Sentence(1..2).fake(),
        amount: 100.0,
        currency: "USD".to_string(),
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
                .text("currency", create_transaction_body.currency.clone()),
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
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateTransactionWithCurrencyAndCategory {
    pub transaction_date: chrono::DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub category_id: i32,
}
#[sqlx::test]
async fn test_create_transaction_success(pool: PgPool) {
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
}

#[sqlx::test]
async fn test_create_transaction_success_not_logged_in(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;

    //Act - Part 1 - Create transaction

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

    assert!(create_transaction_response.status().is_client_error());
}
#[sqlx::test]
async fn test_create_transaction_error_missing_description(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;
    //Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_success());

    let _login_response_body = login_response
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

    assert!(create_transaction_response.status().is_client_error());
}
#[sqlx::test]
async fn test_create_transaction_error_missing_amount(pool: PgPool) {
    let app = spawn_app(pool.clone()).await;
    //Act - Part 1 - Login
    let login_body = LoginUser {
        email: app.test_user.email.clone(),
        password: app.test_user.password.clone(),
    };

    let login_response = app.post_login(&login_body).await;

    assert!(login_response.status().is_success());

    let _login_response_body = login_response
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
                .text("currency", create_transaction_body.currency.clone())
                .text(
                    "category_id",
                    create_transaction_body.category_id.to_string(),
                ),
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(create_transaction_response.status().is_client_error());
}
