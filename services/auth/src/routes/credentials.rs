use crate::{handler::credentials, repository};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(credentials::create::<
                repository::AppLoginHistory,
                repository::AppCredentials,
                repository::AppPasswordReset,
            >))
            .route(web::delete().to(credentials::delete::<
                repository::AppLoginHistory,
                repository::AppCredentials,
                repository::AppPasswordReset,
            >))
            .route(web::put().to(credentials::update_credentials::<
                repository::AppLoginHistory,
                repository::AppCredentials,
                repository::AppPasswordReset,
            >)),
    );
}
