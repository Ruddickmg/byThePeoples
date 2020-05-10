use crate::handler::verification;
use crate::model;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .app_data(web::Json::<model::AuthRequest>)
            .route(
                web::post().to(verification::authenticate_credentials::<model::DatabaseConnection>),
            ),
    );
}
