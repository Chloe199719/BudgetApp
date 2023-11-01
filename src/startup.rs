use actix_cors::Cors;
use actix_web::{ web, cookie, http::header };
use sqlx::postgres;
use sqlx;
use tracing_actix_web::TracingLogger;
use crate::{ settings::Settings, routes::{ health_check, users::auth_routes_config } };
use std::net::TcpListener;
pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(
        settings: Settings,
        test_pool: Option<postgres::PgPool>
    ) -> Result<Self, std::io::Error> {
        let connection_pool = if let Some(pool) = test_pool {
            pool
        } else {
            let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");
            match sqlx::postgres::PgPoolOptions::new().max_connections(5).connect(&db_url).await {
                Ok(pool) => pool,
                Err(e) => {
                    tracing::event!(target: "sqlx",tracing::Level::ERROR, "Couldn't establish DB connection!: {:#?}", e);
                    panic!("Couldn't establish DB connection!")
                }
            }
        };
        sqlx::migrate!().run(&connection_pool).await.expect("Failed to migrate the database.");
        let address = format!("{}:{}", settings.application.host, settings.application.port);

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

async fn run(
    listener: TcpListener,
    db_pool: postgres::PgPool,
    settings: Settings
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
    let redis_store = actix_session::storage::RedisSessionStore
        ::new(redis_url.clone()).await
        .expect("Cannot unwrap redis session.");
    let server = actix_web::HttpServer::new(move || {
        actix_web::App
            ::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&settings.frontend_url)
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    .supports_credentials()
                    .max_age(3600)
            )
            .wrap(TracingLogger::default())
            .wrap(
                actix_session::SessionMiddleware
                    ::builder(redis_store.clone(), secret_key.clone())
                    .cookie_http_only(true)
                    .cookie_same_site(cookie::SameSite::None)
                    .cookie_secure(true)
                    .cookie_name("sessionid".to_string())
                    .build()
            )
            .service(health_check)
            .configure(auth_routes_config)
            .app_data(connection_pool.clone())
            .app_data(redis_pool_data.clone())
        // .wrap(actix_web::middleware::Logger::default())
    });

    if settings.application.protocol == "http" {
        let server = server.listen(listener)?.run();
        return Ok(server);
    } else {
        let mut builder = openssl::ssl::SslAcceptor
            ::mozilla_intermediate(openssl::ssl::SslMethod::tls())
            .unwrap();
        builder
            .set_private_key_file("chloepratas.com.key", openssl::ssl::SslFiletype::PEM)
            .expect("Failed to set private key file.");
        builder
            .set_certificate_chain_file("chloepratas.com.crt")
            .expect("Failed to set certificate chain file");
        let server = server.listen_openssl(listener, builder)?.run();
        return Ok(server);
    }
}
