use crate::{handler::credentials, model, repository};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(credentials::create::<model::DatabaseConnection, repository::AppCredentials>))
            .route(web::delete().to(credentials::delete::<model::DatabaseConnection, repository::AppCredentials>))
            .route(web::put().to(credentials::update_credentials::<model::DatabaseConnection, repository::AppCredentials>)),
    );
}
