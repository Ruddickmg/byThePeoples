use database::configuration;
use serde::{Deserialize, Serialize};
use std::env;
pub mod credentials;

pub type Database = database::DB;

#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub name: String,
    pub password: String,
}

pub struct ServiceState {
    pub db: Database,
}

pub async fn initialize() -> Result<Database, ()> {
    let path_to_migrations = format!(
        "path {}/src/sql/migrations",
        env::current_dir().unwrap().to_str().unwrap()
    );
    println!("path to migrations: {}", path_to_migrations);
    let db_config = configuration::Configuration {
        database: String::from("postgres"),
        password: String::from("password"),
        user: String::from("postgres"),
        host: String::from("127.0.0.3"),
        port: String::from("8080"),
    };
    match database::DB::new(db_config).await {
        Ok(mut db) => {
            if environment::in_development() {
                match db
                    .migrate("/home/moon/web/byThePeoples/services/auth/src/sql/migrations")
                    .await
                {
                    Ok(_) => print!("Migration Successful.\n"),
                    Err(error) => panic!(format!("Error running migrations: {:?}", error)),
                };
            }
            Ok(db)
        }
        Err(error) => panic!("Could not connect to database, Error: {:?}", error),
    }
}
