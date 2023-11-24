mod create_category;
use actix_web::web::ServiceConfig;

pub fn categories_routes_config(cfg: &mut ServiceConfig) {
    cfg.service(actix_web::web::scope("/categories"));
}
