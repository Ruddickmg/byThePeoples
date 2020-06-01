use actix_web::web;

mod credentials;
mod verification;
mod password_reset;

pub const VERIFICATION_ROUTE: &str = "/verify";
pub const CREDENTIALS_ROUTE: &str = "/credentials";
pub const RESET_ROUTE: &str = "/reset";

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope(VERIFICATION_ROUTE).configure(verification::config))
        .service(web::scope(CREDENTIALS_ROUTE).configure(credentials::config))
        .service(web::scope(RESET_ROUTE).configure(password_reset::config));
}
