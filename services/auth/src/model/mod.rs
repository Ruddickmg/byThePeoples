use serde::{Deserialize, Serialize};
use std::{env, sync};

pub mod credentials;

pub type Database = database::DB;

#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub name: String,
    pub password: String,
}

pub struct ServiceState {
    pub db: sync::Mutex<Database>,
}

pub async fn initialize() -> Result<Database, database::Error> {
    let path_to_migrations = format!(
        "{}/src/sql/migrations",
        env::current_dir().unwrap().to_str().unwrap()
    );
    println!("path to migrations: {}", path_to_migrations);
    let db_config = database::Configuration {
        database: String::from("postgres"),
        password: String::from("password"),
        user: String::from("postgres"),
        host: String::from("127.0.0.3"),
        port: String::from("8080"),
    };
    let mut db = database::DB::new(db_config).await?;
    if environment::in_development() {
        db.migrate("/home/moon/web/byThePeoples/services/auth/src/sql/migrations")
            .await?;
        print!("Migration Successful.\n");
    }
    Ok(db)
}
