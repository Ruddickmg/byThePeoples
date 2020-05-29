use actix_web::{web, App, HttpServer};
use crate::{
    configuration::{
        connection,
    },
    model,
    routes,
};

pub async fn development(state: model::AppServiceState) -> std::io::Result<()> {
    use listenfd::ListenFd;
    let uri = connection::uri();
    let data = web::Data::new(state);
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration)
    });
    let mut listen = ListenFd::from_env();
    server = if let Some(listener) = listen.take_tcp_listener(0).unwrap() {
        println!("Hot reloading enabled.");
        server.listen(listener)?
    } else {
        println!("No TCP listener found");
        server.bind(&uri)?
    };
    println!("Development: listening at {}", &uri);
    server.run().await
}

pub async fn production(state: model::AppServiceState) -> std::io::Result<()> {
    let uri = connection::uri();
    let data = web::Data::new(state);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration)
    });
    println!("Production: listening at {}", &uri);
    server.bind(&uri)?.run().await
}