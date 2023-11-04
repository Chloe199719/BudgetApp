use std::sync::Arc;
use serde::{ Serialize, Deserialize };
use serde_json::json;
use tracing::field::{ Field, Visit };
use axiom_rs::Client;
use tokio::spawn;
use tracing::{ Subscriber, Event };
use tracing_subscriber::{ layer::{ SubscriberExt, Context }, Layer };

struct LogData {
    message: String,

    // Add other fields as needed
}

impl Visit for &mut LogData {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
        // Handle other fields as needed
    }
}
use crate::settings;
struct AxiomLayer {
    client: Arc<Client>,
    dataset: String,
}
impl AxiomLayer {
    pub fn new() -> Self {
        tracing::info!("Initializing Axiom telemetry.");
        let settings = settings::get_settings().expect("Failed to load settings.");
        let client = Client::builder()
            .with_token(settings.axiom.token.clone())
            .build()
            .expect("Failed to build Axiom client.");
        Self {
            client: Arc::new(client),
            dataset: settings.axiom.dataset.clone(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct ReturnAxiom {
    message: String,
    metadata: String,
    target: String,
}
impl<S> Layer<S>
    for AxiomLayer
    where S: Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>
{
    fn on_event(&self, event: &Event, _ctx: Context<S>) {
        let mut log_data = LogData {
            message: String::new(),
        };

        // Populate log_data by visiting the fields of the event
        event.record(&mut &mut log_data);

        // Convert log_data into a JSON string
        let json_log = match event.metadata().level() {
            &tracing::Level::ERROR => {
                json!({
                    "error": format!("message - {} | metadata - {} | target - {}",log_data.message,event.metadata().name().to_string(),     
                    event.metadata().target().to_string())
                })
            }
            &tracing::Level::WARN => {
                json!({
                    "warn": format!("message - {} | metadata - {} | target - {}",log_data.message,event.metadata().name().to_string(),     
                    event.metadata().target().to_string())
                })
            }
            &tracing::Level::INFO => {
                json!({
                    "info": format!("message - {} | metadata - {} | target - {}",log_data.message,event.metadata().name().to_string(),     
                    event.metadata().target().to_string())
                })
            }
            &tracing::Level::DEBUG => {
                json!({
                    "debug": format!("message - {} | metadata - {} | target - {}",log_data.message,event.metadata().name().to_string(),     
                    event.metadata().target().to_string())
                })
            }
            &tracing::Level::TRACE => {
                json!({
                    "trace": format!("message - {} | metadata - {} | target - {}",log_data.message,event.metadata().name().to_string(),     
                    event.metadata().target().to_string())
                })
            }
        };

        let client = Arc::clone(&self.client);
        let dataset = self.dataset.clone();
        spawn(async move {
            if let Err(err) = client.ingest(&dataset, vec![json_log]).await {
                eprintln!("Failed to send log to Axiom: {}", err);
            }
        });
    }
}

pub fn get_subscriber(debug: bool) -> impl tracing::Subscriber + Send + Sync {
    let env_filter = if debug { "trace".to_string() } else { "info".to_string() };
    let env_filter = tracing_subscriber::EnvFilter
        ::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(env_filter));
    let env_filter = env_filter.add_directive(
        "actix_http=info".parse().expect("Invalid directive")
    );
    let env_filter = env_filter.add_directive("hyper=info".parse().expect("Invalid directive"));
    let stdout_layer = tracing_subscriber::fmt::layer().pretty();
    let subscriber = tracing_subscriber::Registry::default().with(env_filter).with(stdout_layer);
    let json_log = if !debug {
        let json_log = tracing_subscriber::fmt::layer().json();
        Some(json_log)
    } else {
        None
    };
    let subscriber = subscriber.with(json_log);
    let axiom_layer = AxiomLayer::new();
    let subscriber = subscriber.with(axiom_layer);
    subscriber
}

pub fn init_subscriber(subscriber: impl tracing::Subscriber + Send + Sync) {
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
