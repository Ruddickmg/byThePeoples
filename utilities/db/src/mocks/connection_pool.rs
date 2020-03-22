use super::MockMethod;
use crate::{connection_pool::DatabaseTrait, Client, Database, Result};
use async_trait::async_trait;

pub struct MockConnectionPool {
    pub client: MockMethod<Client>,
    pub migrate: MockMethod<()>,
}

impl MockConnectionPool {
    pub async fn new() -> Result<Database> {
        Ok(Box::new(MockConnectionPool {
            client: mock::Method::new("client"),
            migrate: mock::Method::new("migrate"),
        }))
    }
    pub async fn client(&mut self) -> Result<Client> {
        self.client.call()
    }
    pub async fn migrate(&mut self, _: &str) -> Result<()> {
        self.migrate.call()
    }
}

#[async_trait]
impl<'a> DatabaseTrait for MockConnectionPool {
    async fn client(&'a self) -> Result<Client> {
        self.client().await
    }
    async fn migrate(&self, path: &str) -> Result<()> {
        self.migrate(path).await
    }
}
