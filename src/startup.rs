use crate::{
    routes::{
        categories::categories_routes_config, health_check,
        transactions::transactions_routes_config, users::auth_routes_config,
    },
    settings::Settings,
    uploads,
};
use actix_cors::Cors;
use actix_web::{cookie, http::header, web};
use aws_sdk_s3::config::{Credentials, Region};
use sqlx;
use sqlx::postgres;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(
        settings: Settings,
        test_pool: Option<postgres::PgPool>,
    ) -> Result<Self, std::io::Error> {
        let connection_pool = if let Some(pool) = test_pool {
            pool
        } else {
            let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");
            match sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&db_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => {
                    tracing::event!(target: "sqlx",tracing::Level::ERROR, "Couldn't establish DB connection!: {:#?}", e);
                    panic!("Couldn't establish DB connection!")
                }
            }
        };
        sqlx::migrate!()
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database.");
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, settings).await?;

        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn configure_and_return_s3_client() -> uploads::client::Client {
    // S3 client configuration
    // Get id and secret key from environment variables
    let aws_key = std::env::var("AWS_ACCESS_KEY_ID").expect("Failed to get AWS_ACCESS_KEY_ID.");
    let aws_key_secret =
        std::env::var("AWS_SECRET_ACCESS_KEY").expect("Failed to get AWS_SECRET_ACCESS_KEY.");

    let aws_cred = Credentials::new(
        aws_key,
        aws_key_secret,
        None,
        None,
        "loaded-from-custom-env",
    );
    let aws_region = Region::new(std::env::var("AWS_REGION").unwrap_or("eu-central-1".to_string()));
    let aws_config_builder = aws_sdk_s3::config::Builder::new()
        .region(aws_region)
        .credentials_provider(aws_cred);
    let aws_config = aws_config_builder.build();
    uploads::client::Client::new(aws_config)
}

async fn run(
    listener: TcpListener,
    db_pool: postgres::PgPool,
    settings: Settings,
) -> Result<actix_web::dev::Server, std::io::Error> {
    // Database connection pool applications state
    let connection_pool = web::Data::new(db_pool);

    // Redis connection pool applications state
    let redis_url = std::env::var("REDIS_URL").expect("Failed to get REDIS_URL.");
    let cfg = deadpool_redis::Config::from_url(redis_url.clone());
    let redis_pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool.");
    let redis_pool_data = web::Data::new(redis_pool);

    let secret_key = cookie::Key::from(settings.secret.hmac_secret.as_bytes());
    let redis_store = actix_session::storage::RedisSessionStore::new(redis_url.clone())
        .await
        .expect("Cannot unwrap redis session.");

    // S3 client configuration
    let s3_client = actix_web::web::Data::new(configure_and_return_s3_client().await);

    // Server configuration
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&settings.frontend_url)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(TracingLogger::default())
            .wrap(
                actix_session::SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(cookie::SameSite::Lax)
                    .cookie_secure(true)
                    .cookie_name("sessionid".to_string())
                    .build(),
            )
            .service(health_check)
            .configure(auth_routes_config)
            .configure(categories_routes_config)
            .configure(transactions_routes_config)
            .app_data(connection_pool.clone())
            .app_data(redis_pool_data.clone())
            .app_data(s3_client.clone())
    })
    .workers(16);

    // Server protocol configuration
    if settings.application.protocol == "http" {
        let server = server.listen(listener)?.run();
        Ok(server)
    } else {
        let mut builder =
            openssl::ssl::SslAcceptor::mozilla_intermediate(openssl::ssl::SslMethod::tls())
                .unwrap();
        builder
            .set_private_key_file("chloepratas.com.key", openssl::ssl::SslFiletype::PEM)
            .expect("Failed to set private key file.");
        builder
            .set_certificate_chain_file("chloepratas.com.crt")
            .expect("Failed to set certificate chain file");
        let server = server.listen_openssl(listener, builder)?.run();
        Ok(server)
    }
}
