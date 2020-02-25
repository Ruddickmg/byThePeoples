use actix_web::{web, App, HttpServer};
use btp_api_server::{connection, graph_ql, routes, AppData};
use std::sync::Mutex;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let uri = connection::uri();
    let data = web::Data::new(AppData {
        schema: graph_ql::graph_schema(),
        app_name: String::from("ByThePeoples"),
        counter: Mutex::new(0),
    });

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration)
    });

    if environment::in_production() {
        println!("in production");
        server = server.bind(&uri)?;
    } else {
        use listenfd::ListenFd;
        let mut listen = ListenFd::from_env();

        println!("Running in development mode.");

        server = if let Some(listener) = listen.take_tcp_listener(0).unwrap() {
            println!("Hot reloading enabled.");
            server.listen(listener)?
        } else {
            server.bind(&uri)?
        };
    };

    println!("Listening at {}", &uri);

    server.run().await
}
