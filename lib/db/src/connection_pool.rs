use crate::{client, configuration, Configuration, Pool, Result};
use async_trait::async_trait;
use std::fs;

const SQL_EXTENSION: &str = "sql";

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool,
}

impl ConnectionPool {
    pub async fn new(cfg: Configuration) -> Result<ConnectionPool> {
        let manager =
            bb8_postgres::PostgresConnectionManager::new(cfg.build()?, tokio_postgres::tls::NoTls);
        let pool: Pool = bb8::Pool::builder()
            .max_size(configuration::POOL_SIZE)
            .build(manager)
            .await?;
        if environment::in_development() {
            println!("Connected to database.");
        }
        Ok(ConnectionPool { pool })
    }
    pub async fn client(&self) -> Result<client::Client<'_>> {
        Ok(client::Client::new(self.pool.get().await?))
    }
    pub async fn migrate(&self, path: &str) -> Result<()> {
        let mut sql_files = files::by_extension(path, SQL_EXTENSION);
        sql_files.sort();
        let client = self.client().await?;
        for file_path in sql_files.iter() {
            let sql = fs::read_to_string(file_path)?;
            client.batch(&sql).await?;
        }
        Ok(())
    }
}

#[async_trait]
pub trait ConnectionPoolTrait: Clone {
    async fn client(&self) -> Result<client::Client<'_>>;
    async fn migrate(&self, path: &str) -> Result<()>;
}

#[async_trait]
impl ConnectionPoolTrait for ConnectionPool {
    async fn client(&self) -> Result<client::Client<'_>> {
        Ok(client::Client::new(self.pool.get().await?))
    }
    async fn migrate(&self, path: &str) -> Result<()> {
        let mut sql_files = files::by_extension(path, SQL_EXTENSION);
        sql_files.sort();
        let client = self.client().await?;
        for file_path in sql_files.iter() {
            let sql = fs::read_to_string(file_path)?;
            client.batch(&sql).await?;
        }
        Ok(())
    }
}
