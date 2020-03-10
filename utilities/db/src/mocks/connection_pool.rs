use super::MockMethod;
use crate::{connection_pool::DatabaseTrait, Client, Database, Result};
use async_trait::async_trait;

pub struct MockConnectionPool<'a> {
    pub client: MockMethod<Client<'a>>,
    pub migrate: MockMethod<()>,
}

impl<'a> MockConnectionPool<'a> {
    pub async fn new() -> Result<Database<'a>> {
        Ok(Box::new(MockConnectionPool {
            client: mock::Method::new("client"),
            migrate: mock::Method::new("migrate"),
        }))
    }
    pub async fn client(&'a mut self) -> Result<Client<'a>> {
        self.client.call()
    }
    pub async fn migrate(&'a mut self, _: &str) -> Result<()> {
        self.migrate.call()
    }
}

#[async_trait]
impl<'a> DatabaseTrait<'a> for MockConnectionPool<'a> {
    async fn client(&'a self) -> Result<Client<'a>> {
        self.client().await
    }
    async fn migrate(&self, path: &str) -> Result<()> {
        self.migrate(path).await
    }
}
