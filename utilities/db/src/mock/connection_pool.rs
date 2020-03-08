use super::Mocker;
use crate::{client, connection_pool::DatabaseTrait, Client, Configuration, Database, Error};
use async_trait::async_trait;

#[async_trait]
impl DatabaseTrait for MockConnectionPool {
    async fn client(&self) -> Result<Client<'_>, Error> {
        self.client().await
    }
    async fn migrate(&self, path: &str) -> Result<(), Error> {
        self.migrate(path).await
    }
}

pub struct MockConnectionPool {
    pub client: Mocker<Client<'static>, (), Error>,
    pub migrate: Mocker<(), String, Error>,
}

impl MockConnectionPool {
    pub async fn new() -> Result<Database, Error> {
        Ok(Box::new(MockConnectionPool {
            client: Mocker::new("client"),
            migrate: Mocker::new("migrate"),
        }))
    }
    pub async fn client(&self) -> Result<Client<'static>, Error> {
        self.client.call(None)
    }
    pub async fn migrate(&self, path: &str) -> Result<(), Error> {
        self.migrate.call(Some(String::from(path)))
    }
}
