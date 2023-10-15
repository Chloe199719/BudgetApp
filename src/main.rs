use actix_web::{ get, post, web, App, HttpServer };
use discord_backend::{ settings, telemetry, startup::Application };
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let settings = settings::get_settings().expect("Failed to load settings.");
    let subscriber = telemetry::get_subscriber(settings.clone().debug);
    telemetry::init_subscriber(subscriber);
    let application = Application::build(settings).await?;
    tracing::event!(target:"discord_backend", tracing::Level::INFO, "Listening on http://127.0.0.1:{}/", application.port());
    application.run_until_stopped().await?;
    Ok(())
}
