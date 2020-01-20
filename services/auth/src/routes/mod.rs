use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

mod auth;

pub struct AppStateWithCounter {
    pub app_name: String,
    pub counter: Mutex<i32>,
}

async fn hello_world(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok()
        .body(format!("Welcome to {}! you're visitor #{}", &data.app_name, counter))
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/").route(web::get().to(hello_world)))
        .service(web::scope("/model").configure(auth::config));
}