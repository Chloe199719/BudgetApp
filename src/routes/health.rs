use actix_web::{ get, HttpResponse };

#[tracing::instrument]
#[get("/health-check")]
pub async fn health_check() -> HttpResponse {
    tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing health-check endpoint.");
    HttpResponse::Ok().json("Application is safe and healthy.")
}
