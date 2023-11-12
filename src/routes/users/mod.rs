pub mod register;
pub mod confirm_registration;
pub mod login;
pub mod logout;
mod update_user;

pub fn auth_routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web
            ::scope("/users")
            .service(register::register_user)
            .service(confirm_registration::config)
            .service(login::login_user)
            .service(logout::logout_user)
    );
}
