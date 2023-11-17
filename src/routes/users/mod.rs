pub mod confirm_registration;
mod current_user;
mod generate_new_token;
pub mod login;
pub mod logout;
mod password_change;
pub mod register;
mod update_user;

pub fn auth_routes_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/users")
            .service(register::register_user)
            .service(confirm_registration::config)
            .service(login::login_user)
            .service(logout::logout_user)
            .service(update_user::update_users_details)
            .service(current_user::get_current_user)
            .service(generate_new_token::regenerate_token)
            .service(password_change::request_change::request_password_change)
            .service(password_change::confirm_change_request::confirm_change_password_token)
            .service(password_change::change::change_user_password),
    );
}
