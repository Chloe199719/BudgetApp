pub mod register;
pub mod confirm_registration;
pub mod login;
pub mod logout;
mod update_user;
mod current_user;
mod generate_new_token;

pub fn auth_routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web
            ::scope("/users")
            .service(register::register_user)
            .service(confirm_registration::config)
            .service(login::login_user)
            .service(logout::logout_user)
            .service(update_user::update_users_details)
            .service(current_user::get_current_user)
            .service(generate_new_token::regenerate_token)
    );
}
