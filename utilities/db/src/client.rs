use crate::{
    transaction::{GenericTransaction, Transaction},
    Params, PooledConnection, Result, Results, Statement,
};
use async_trait::async_trait;
use std::fs;

pub struct Client<'b> {
    client: PooledConnection<'b>,
}

pub type GenericClient<'a> = Box<dyn ClientTrait<'a> + 'a + Send + Sync>;

#[async_trait]
pub trait ClientTrait<'b> {
    async fn execute<'a>(&self, query: &str, params: Params<'a>) -> Result<u64>;
    async fn prepare(&self, query: &str) -> Result<Statement>;
    async fn query<'a>(&self, stmt: &Statement, params: Params<'a>) -> Result<Results>;
    async fn batch(&mut self, sql: &str) -> Result<()>;
    async fn transaction(&'b mut self) -> Result<GenericTransaction<'_>>;
    async fn execute_file(&mut self, path: &str) -> Result<()>;
}

#[async_trait]
impl<'b> ClientTrait<'b> for Client<'b> {
    async fn execute<'a>(&self, query: &str, params: Params<'a>) -> Result<u64> {
        self.execute(query, params).await
    }
    async fn prepare(&self, query: &str) -> Result<Statement> {
        self.prepare(query).await
    }

    async fn query<'a>(&self, query: &Statement, params: Params<'a>) -> Result<Results> {
        self.query(query, params).await
    }

    async fn batch(&mut self, sql: &str) -> Result<()> {
        self.batch(sql).await
    }

    async fn transaction(&'b mut self) -> Result<GenericTransaction<'_>> {
        self.transaction().await
    }

    async fn execute_file(&mut self, path: &str) -> Result<()> {
        self.execute_file(path).await
    }
}

impl<'b> Client<'b> {
    pub fn new(client: PooledConnection<'b>) -> GenericClient<'b> {
        Box::new(Client { client })
    }
    pub async fn execute<'a>(self, query: &str, params: Params<'a>) -> Result<u64> {
        Ok(self.client.execute(query, params).await?)
    }
    pub async fn prepare(self, query: &str) -> Result<Statement> {
        Ok(self.client.prepare(query).await?)
    }
    pub async fn query<'a>(self, query: &Statement, params: Params<'a>) -> Result<Results> {
        Ok(self.client.query(query, params).await?)
    }
    pub async fn transaction(&'b mut self) -> Result<GenericTransaction<'_>> {
        Ok(Transaction::new(self.client.transaction().await?).await?)
    }
    pub async fn batch(&mut self, sql: &str) -> Result<()> {
        Ok(self.client.batch_execute(sql).await?)
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<()> {
        Ok(self.batch(&fs::read_to_string(path)?).await?)
    }
}
