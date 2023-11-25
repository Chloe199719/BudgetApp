pub mod create_category;
pub mod delete_category;
use actix_web::web::ServiceConfig;

pub fn categories_routes_config(cfg: &mut ServiceConfig) {
    cfg.service(actix_web::web::scope("/categories").service(create_category::create_category));
}
