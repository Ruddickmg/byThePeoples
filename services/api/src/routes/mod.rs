use actix_web::{web, HttpResponse, Responder};

mod graph_ql;
mod user;

async fn hello_world(data: web::Data<super::AppData>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!(
        "Welcome to {}! you're visitor #{}",
        &data.app_name, counter
    ))
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(hello_world)))
        .service(web::scope("").configure(graph_ql::configuration))
        .service(web::scope("/model").configure(user::config));
}
