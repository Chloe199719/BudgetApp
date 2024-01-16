pub mod change_budget_amount;
pub mod change_budget_date;
pub mod change_budget_recursing;
pub mod create_budget;
pub mod delete_budget;
use actix_web::web::ServiceConfig;

pub fn budget_routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/budgets")
            .service(create_budget::create_budget)
            .service(delete_budget::delete_budget_route)
            .service(change_budget_amount::change_budget_amount)
            .service(change_budget_date::change_budget_date_route)
            .service(change_budget_recursing::change_budget_recursing),
    );
}
