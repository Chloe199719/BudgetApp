pub mod create_transaction;
pub mod get_all_transactions_by_user;

use actix_web::web::ServiceConfig;

pub fn transactions_routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/transactions")
            .service(create_transaction::create_transaction)
            .service(get_all_transactions_by_user::get_all_transactions_by_user),
    );
}
