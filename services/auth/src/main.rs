use actix_web::{web, App, HttpServer};
use btp_auth_server::{connection, model, routes};
use environment;

const APP_STATE_CREATION_FAILURE: &str = "Failed to create application state";
const APP_STATE_INITIALIZATION_FAILURE: &str = "Failed to initialize the application state";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = model::ServiceState::new()
        .await
        .expect(APP_STATE_CREATION_FAILURE)
        .initialize()
        .await
        .expect(APP_STATE_INITIALIZATION_FAILURE);
    let uri = connection::uri();
    let data = web::Data::new(state);
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
