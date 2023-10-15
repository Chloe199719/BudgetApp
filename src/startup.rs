use crate::{ settings::Settings, routes::health_check };
use std::net::TcpListener;
pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(settings: Settings) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", settings.application.host, settings.application.port);

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener).await?;

        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = actix_web::HttpServer
        ::new(|| { actix_web::App::new().service(health_check) })
        .listen(listener)?
        .run();
    Ok(server)
}
