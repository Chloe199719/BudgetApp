use budget_app::{
    routes::transactions::delete_transaction::delete_transaction,
    types::{
        transactions::create::{TransactionOutcomeWithReceipt, TransactionType},
        UserVisible, general::SuccessResponse,
    },
};
use chrono::Utc;
use fake::{faker::lorem::en::Sentence, Fake};
use reqwest::multipart::Form;
use sqlx::PgPool;

use crate::{
    helpers::spawn_app,
    transactions::create_a_transaction::CreateTransactionWithCurrencyAndCategory,
    users::login::LoginUser,
};

#[sqlx::test]
async fn delete_transaction_by_id_success(pool: PgPool) {
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
    let create_transaction_body2 = CreateTransactionWithCurrencyAndCategory {
        transaction_date: Utc::now(),
        transaction_type: TransactionType::WITHDRAWAL,
        description: Sentence(1..2).fake(),
        amount: 100.0,
        currency: "USD".to_string(),
        category_id: 1,
    };

    let create_transaction_response2 = app
        .api_client
        .post(&format!("{}/transactions/create", app.address))
        .multipart(
            Form::new()
                .text(
                    "transaction_date",
                    create_transaction_body2.transaction_date.to_rfc3339(),
                )
                .text(
                    "transaction_type",
                    create_transaction_body2.transaction_type.to_string(),
                )
                .text("description", create_transaction_body2.description.clone())
                .text("amount", create_transaction_body2.amount.to_string())
                .text("currency", create_transaction_body2.currency.clone())
                .text(
                    "category_id",
                    create_transaction_body2.category_id.to_string(),
                ),
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(create_transaction_response2.status().is_success());

    let create_transaction_response_body2 = create_transaction_response2
        .json::<budget_app::types::transactions::create::TransactionOutcomeWithReceipt>()
        .await
        .expect("Failed to parse create transaction response");

    assert_eq!(
        create_transaction_response_body2.transaction_type,
        "WITHDRAWAL".to_string()
    );
    assert_eq!(
        create_transaction_response_body2.description,
        create_transaction_body2.description
    );
    assert_eq!(
        create_transaction_response_body2.amount,
        create_transaction_body2.amount
    );
    assert_eq!(
        create_transaction_response_body2.user_id,
        login_response_body.id
    );
    assert_eq!(
        create_transaction_response_body2.date.date_naive(),
        create_transaction_body2.transaction_date.date_naive()
    );
    assert_eq!(
        create_transaction_response_body2.currency.to_string(),
        create_transaction_body2.currency.to_string()
    );
    assert_eq!(
        create_transaction_response_body2.category_id,
        create_transaction_body2.category_id
    );
    let delete_transaction_response = app
        .api_client
        .delete(&format!(
            "{}/transactions/delete/{}",
            app.address, create_transaction_response_body.transaction_id
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(delete_transaction_response.status().is_success());

    let delete_transaction_response_body = delete_transaction_response
        .json::<SuccessResponse>()
        .await
        .expect("Failed to parse delete transaction response");
    assert_eq!(
        delete_transaction_response_body.message,
        "Transaction deleted successfully"
    );

    
        let get_transaction_by_id_first = app
        .api_client
        .get(&format!(
            "{}/transactions/get_transaction_by_id/{}",
            app.address, create_transaction_response_body.transaction_id
        ))
        .send()
        .await
        .expect("Failed to execute request.");

        assert!(get_transaction_by_id_first.status().is_client_error())
}
