use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("").route(web::get().to(|| HttpResponse::Ok().body("do some stupid stuff!"))),
    );
}
