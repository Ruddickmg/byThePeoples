use actix_web::web;
mod auth;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").configure(auth::config));
}
