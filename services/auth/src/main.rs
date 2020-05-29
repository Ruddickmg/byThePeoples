use environment;
use btp_auth_server::{
    configuration::{
        database::TEST_DATABASE_CONFIG,
    },
    server,
    model,
};

const DATABASE_INITIALIZATION_FAILURE: &str = "Failed to initialize database";

pub async fn run_migrations<T: model::Database>(db: &T) -> std::result::Result<(), database::Error> {
    let path_to_migrations = environment::path("/src/sql/migrations");
    db.migrate(&path_to_migrations).await?;
    println!("Migration Successful.\n");
    Ok(())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
        .await
        .expect(DATABASE_INITIALIZATION_FAILURE);
    let state = model::initialize_state(&db);
    if environment::in_production() {
        println!("In production");
        server::production(state.clone())
            .await
    } else {
        println!("In development");
        run_migrations(&db)
            .await
            .expect(DATABASE_INITIALIZATION_FAILURE);
        server::development(state.clone()).await
    }
}
