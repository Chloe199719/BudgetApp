pub mod create_category;
pub mod delete_category;
pub mod edit;
pub mod get_category_by_id;
use actix_web::web::ServiceConfig;

pub fn categories_routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        actix_web::web
            ::scope("/categories")
            .service(create_category::create_category)
            .service(delete_category::delete_category)
            .service(edit::edit_category)
    );
}
