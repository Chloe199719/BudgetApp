use actix_web::{ get, post, web, App, HttpServer };

#[actix_web::main]
async fn main() {
    HttpServer::new(|| { App::new() })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run().await
        .unwrap();
}
