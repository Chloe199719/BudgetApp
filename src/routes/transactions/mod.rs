use actix_web::web::ServiceConfig;

pub fn transactions_routes_config(cfg: &mut ServiceConfig) {
    cfg.service(actix_web::web::scope("/transactions"));
}
