use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use budget_app::{
    settings::get_settings,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use reqwest::{redirect::Policy, Client, Response};
use serde::Serialize;
use sqlx::PgPool;
use super::categories::TEST_CATEGORY_DESCRIPTION;
use super::categories::TEST_CATEGORY_NAME;
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber(false);
    init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub test_user: TestUser,
    pub api_client: Client,
}

impl TestApp {
    pub async fn post_login<Body>(&self, body: &Body) -> Response
    where
        Body: Serialize,
    {
        self.api_client
            .post(&format!("{}/users/login/", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub async fn spawn_app(pool: PgPool) -> TestApp {
    dotenv().ok();

    Lazy::force(&TRACING);
    let settigs = {
        let mut s = get_settings().expect("Failed to read settings.");

        //Use a Random OS Port
        s.application.port = 0;
        s
    };

    let application = Application::build(settigs.clone(), Some(pool.clone()))
        .await
        .expect("Failed to build application.");

    let address = format!("http://127.0.0.1:{}", application.port());

    let _ = tokio::spawn(application.run_until_stopped());

    let client = Client::builder()
        .redirect(Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    let test_app = TestApp {
        address,
        api_client: client,
        test_user: TestUser::generate().await,
    };

    test_app.test_user.store(&pool).await;

    test_app
}

pub struct TestUser {
    pub email: String,
    pub password: String,
    unique_name: String,
    display_name: String,
}

impl TestUser {
    pub async fn generate() -> Self {
        Self {
            email: uuid::Uuid::new_v4().to_string(),
            password: uuid::Uuid::new_v4().to_string(),
            unique_name: uuid::Uuid::new_v4().to_string(),
            display_name: uuid::Uuid::new_v4().to_string(),
        }
    }
    #[rustfmt::skip]
    async fn store(&self, pool: &PgPool){
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = Argon2::default()
            .hash_password(&self.password.as_bytes(), &salt)
            .expect("Unable to hash password.")
            .to_string();

        let user_id = sqlx::query!(
            "INSERT INTO users (email, password, unique_name, display_name,is_active, is_staff, is_superuser) VALUES ($1, $2, $3, $4,true,true,true) RETURNING id",
            &self.email,
            password_hash,
            &self.unique_name,
            &self.display_name,
            )
            .fetch_one(pool)
            .await
            .expect("Failed to insert test user.");
        sqlx::query!(
            "INSERT INTO user_profile (user_id)
                    VALUES ($1)
                    ON CONFLICT (user_id) DO NOTHING",user_id.id)
            .execute(pool)
            .await
            .expect("Failed to insert test user profile.");
        sqlx::query!(
            "INSERT INTO categories (category_name, description, user_id, is_default)
                    VALUES ($1, $2, $3 , $4)",
                    TEST_CATEGORY_NAME,
                    TEST_CATEGORY_DESCRIPTION,
                    user_id.id,
                    true)
                    .execute(pool)
                    .await
                    .expect("Failed to insert test category.");
                    
        
    }
}
