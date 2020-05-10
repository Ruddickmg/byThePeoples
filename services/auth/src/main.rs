use actix_web::{web, App, HttpServer};
use btp_auth_server::{configuration::database::TEST_DATABASE_CONFIG, connection, model, routes};
use environment;

const DATABASE_INITIALIZATION_FAILURE: &str = "Failed to initialize database";
const APP_STATE_CREATION_FAILURE: &str = "Failed to create application state";
const APP_STATE_INITIALIZATION_FAILURE: &str = "Failed to initialize application state";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
        .await
        .expect(DATABASE_INITIALIZATION_FAILURE);
    let state = model::ServiceState::new(db)
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
