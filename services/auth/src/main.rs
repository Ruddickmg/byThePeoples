use actix_web::{web, App, HttpServer};
use btp_auth_server::{configuration::database::TEST_DATABASE_CONFIG, connection, model, routes};
use environment;
use std::env;

const DATABASE_INITIALIZATION_FAILURE: &str = "Failed to initialize database";

pub async fn run_migrations<T: model::Database>(db: &T) -> Result<(), database::Error> {
    let path_to_migrations = format!(
        "{}/src/sql/migrations",
        env::current_dir().unwrap().to_str().unwrap()
    );
    db.migrate(&path_to_migrations).await?;
    println!("Migration Successful.\n");
    Ok(())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
        .await
        .expect(DATABASE_INITIALIZATION_FAILURE);
    let uri = connection::uri();
    let data = web::Data::new(model::initialize_state(&db));
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
        run_migrations(&db)
            .await
            .expect(DATABASE_INITIALIZATION_FAILURE);
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
