use crate::{client, configuration, Client, Configuration, Database, Pool, Result};
use async_trait::async_trait;
use std::fs;

const SQL_EXTENSION: &str = "sql";

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool,
}

#[async_trait]
pub trait DatabaseTrait {
    async fn client(&mut self) -> Result<Client<'_>>;
    async fn migrate(&mut self, path: &str) -> Result<()>;
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
        Ok(Box::new(ConnectionPool { pool }))
    }
    pub async fn client(&mut self) -> Result<Client<'_>> {
        Ok(client::Client::new(self.pool.get().await?))
    }
    pub async fn migrate(&mut self, path: &str) -> Result<()> {
        let sql_files = files::by_extension(path, SQL_EXTENSION);
        let mut client = self.client().await?;
        for file_path in sql_files.iter() {
            let sql = fs::read_to_string(file_path)?;
            client.batch(&sql).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl DatabaseTrait for ConnectionPool {
    async fn client(&mut self) -> Result<Client<'_>> {
        self.client().await
    }
    async fn migrate(&mut self, path: &str) -> Result<()> {
        self.migrate(path).await
    }
}
