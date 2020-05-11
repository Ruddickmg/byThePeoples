use crate::handler::credentials;
use crate::model;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(credentials::create::<model::DatabaseConnection>))
            .route(web::delete().to(credentials::delete::<model::DatabaseConnection>))
            .route(web::put().to(credentials::update_credentials::<model::DatabaseConnection>)),
    );
}
