use actix_web::web;
use sqlx::postgres;
use sqlx;
use crate::{ settings::{ Settings, DatabaseSettings }, routes::health_check };
use std::{ net::TcpListener, time::Duration };
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
            get_connection_pool(&settings.database).await
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
pub async fn get_connection_pool(settings: &DatabaseSettings) -> postgres::PgPool {
    postgres::PgPoolOptions
        ::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy_with(settings.connect_to_db())
}

async fn run(
    listener: TcpListener,
    db_pool: postgres::PgPool,
    settings: Settings
) -> Result<actix_web::dev::Server, std::io::Error> {
    // Database connection pool applications state
    let connection_pool = web::Data::new(db_pool);

    // Redis connection pool applications state
    let cfg = deadpool_redis::Config::from_url(&settings.redis.uri);
    let redis_pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed to create Redis pool.");
    let redis_pool_data = web::Data::new(redis_pool);

    let server = actix_web::HttpServer
        ::new(move || {
            actix_web::App
                ::new()
                .service(health_check)
                .app_data(connection_pool.clone())
                .app_data(redis_pool_data.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}
