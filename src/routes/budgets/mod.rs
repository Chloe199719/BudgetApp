pub mod create_budget;
use actix_web::web::ServiceConfig;

pub fn budget_routes_config(cfg: &mut ServiceConfig) {
    cfg.service(actix_web::web::scope("/budgets").service(create_budget::create_budget));
}
