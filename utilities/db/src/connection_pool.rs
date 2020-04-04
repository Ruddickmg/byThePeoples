use crate::{client, configuration, Client, Configuration, Database, Pool, Result};
use std::fs;

const SQL_EXTENSION: &str = "sql";

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool,
}

impl ConnectionPool {
    pub async fn new(cfg: Configuration) -> Result<Database> {
        let manager =
            bb8_postgres::PostgresConnectionManager::new(cfg.build()?, tokio_postgres::tls::NoTls);
        let pool: Pool = bb8::Pool::builder()
            .max_size(configuration::POOL_SIZE)
            .build(manager)
            .await?;
        if !environment::in_production() {
            println!("Connected to database.");
        }
        Ok(ConnectionPool { pool })
    }
    pub async fn client(&self) -> Result<Client<'_>> {
        Ok(client::Client::new(self.pool.get().await?))
    }
    pub async fn migrate(&self, path: &str) -> Result<()> {
        let sql_files = files::by_extension(path, SQL_EXTENSION);
        println!("files: {:#?}", sql_files);
        let client = self.client().await?;
        for file_path in sql_files.iter() {
            let sql = fs::read_to_string(file_path)?;
            println!("sql {}", sql);
            client.batch(&sql).await?;
        }
        Ok(())
    }
}
