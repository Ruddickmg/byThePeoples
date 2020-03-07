use crate::{transaction::Transaction, Error, Params, PooledConnection, Results, Statement};
use async_trait::async_trait;
use std::fs;

pub struct Client<'b> {
    client: PooledConnection<'b>,
}

#[async_trait]
pub trait GenericClient<'b> {
    async fn execute<'a>(self, query: &str, params: Params<'a>) -> Result<u64, Error>;
    async fn prepare(self, query: &str) -> Result<Statement, Error>;
    async fn query<'a>(self, stmt: &Statement, params: Params<'a>) -> Result<Results, Error>;
    async fn batch(&mut self, sql: &str) -> Result<(), Error>;
    async fn transaction(&'b mut self) -> Result<Transaction<'_>, Error>;
    async fn execute_file(&mut self, path: &str) -> Result<(), Error>;
}

#[async_trait]
impl<'b> GenericClient<'b> for Client<'b> {
    async fn execute<'a>(self, query: &str, params: Params<'a>) -> Result<u64, Error> {
        self.execute(query, params).await
    }
    async fn prepare(self, query: &str) -> Result<Statement, Error> {
        self.prepare(query).await
    }

    async fn query<'a>(self, query: &Statement, params: Params<'a>) -> Result<Results, Error> {
        self.query(query, params).await
    }

    async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        self.batch(sql).await
    }

    async fn transaction(&'b mut self) -> Result<Transaction<'_>, Error> {
        self.transaction().await
    }

    async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        self.execute_file(path).await
    }
}

impl<'b> Client<'b> {
    pub fn new(client: PooledConnection<'b>) -> Client<'b> {
        Client { client }
    }
    pub async fn execute<'a>(self, query: &str, params: Params<'a>) -> Result<u64, Error> {
        Ok(self.client.execute(query, params).await?)
    }
    pub async fn prepare(self, query: &str) -> Result<Statement, Error> {
        Ok(self.client.prepare(query).await?)
    }
    pub async fn query<'a>(self, query: &Statement, params: Params<'a>) -> Result<Results, Error> {
        Ok(self.client.query(query, params).await?)
    }
    pub async fn transaction(&'b mut self) -> Result<Transaction<'_>, Error> {
        Ok(Transaction::new(self.client.transaction().await?).await?)
    }
    pub async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        Ok(self.client.batch_execute(sql).await?)
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        Ok(self.batch(&fs::read_to_string(path)?).await?)
    }
}
