use actix_web::web;
mod credentials;
// mod verification;

pub const VERIFICATION_ROUTE: &str = "/verify";
pub const CREDENTIALS_ROUTE: &str = "/credentials";

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg
        // .service(web::scope(VERIFICATION_ROUTE).configure(verification::config))
        .service(web::scope(CREDENTIALS_ROUTE).configure(credentials::config));
}
