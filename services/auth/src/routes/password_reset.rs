use crate::{handler::password_reset, repository};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(password_reset::request_password_reset::<
                repository::AppLoginHistory,
                repository::AppCredentials,
                repository::AppPasswordReset,
            >))
            .route(web::put().to(password_reset::reset_password::<
                repository::AppLoginHistory,
                repository::AppCredentials,
                repository::AppPasswordReset,
            >)),
    );
}
