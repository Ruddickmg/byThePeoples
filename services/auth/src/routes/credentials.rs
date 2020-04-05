use crate::handler::credentials;
use crate::model;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .app_data(web::Json::<model::AuthRequest>)
            .route(web::post().to(credentials::create))
            .route(web::delete().to(credentials::delete))
            .route(web::put().to(credentials::update)),
    );
}
