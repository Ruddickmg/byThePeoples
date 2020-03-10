use crate::{client, configuration, Client, Configuration, Database, Pool, Result};
use async_trait::async_trait;
use std::fs;

const SQL_EXTENSION: &str = "sql";

#[derive(Clone)]
pub struct ConnectionPool<'a> {
    pool: Pool,
    phantom: &'a str,
}

#[async_trait]
pub trait DatabaseTrait<'a> {
    async fn client(&'a self) -> Result<Client<'a>>;
    async fn migrate(&self, path: &str) -> Result<()>;
}

impl<'a> ConnectionPool<'a> {
    pub async fn new(cfg: Configuration) -> Result<Database<'a>> {
        let manager =
            bb8_postgres::PostgresConnectionManager::new(cfg.build()?, tokio_postgres::tls::NoTls);
        let pool: Pool = bb8::Pool::builder()
            .max_size(configuration::POOL_SIZE)
            .build(manager)
            .await?;
        if !environment::in_production() {
            println!("Connected to database.");
        }
        Ok(Box::new(ConnectionPool {
            pool,
            phantom: "placeholder",
        }))
    }
    pub async fn client(&'a self) -> Result<Client<'a>> {
        Ok(client::Client::new(self.pool.get().await?))
    }
    pub async fn migrate(&self, path: &str) -> Result<()> {
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
impl<'a> DatabaseTrait<'a> for ConnectionPool<'a> {
    async fn client(&'a self) -> Result<Client<'a>> {
        self.client().await
    }
    async fn migrate(&self, path: &str) -> Result<()> {
        self.migrate(path).await
    }
}
