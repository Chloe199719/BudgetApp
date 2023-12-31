pub mod create_transaction;
pub mod delete_transaction;
pub mod get_all_transactions_by_categorie;
pub mod get_all_transactions_by_user;
pub mod get_transaction_by_id;
pub mod swap_transaction_category;
pub mod update_transaction;

use actix_web::web::ServiceConfig;

pub fn transactions_routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/transactions")
            .service(create_transaction::create_transaction)
            .service(get_all_transactions_by_user::get_all_transactions_by_user)
            .service(get_all_transactions_by_categorie::get_all_transactions_by_category)
            .service(get_transaction_by_id::get_transaction_by_id)
            .service(delete_transaction::delete_transaction)
            .service(update_transaction::update_transaction_route),
    );
}
