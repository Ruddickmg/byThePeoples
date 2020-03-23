use crate::{
    transaction::{GenericTransaction, Transaction},
    Params, PooledConnection, Result, Results, Statement,
};
use async_trait::async_trait;
use std::fs;

pub struct Client<'a> {
    connection: PooledConnection<'a>,
}

pub type GenericClient<'a> = Box<dyn ClientTrait<'a> + 'a + Send + Sync>;

#[async_trait]
pub trait ClientTrait<'a> {
    async fn execute<'b>(&self, query: &str, params: Params<'b>) -> Result<u64>;
    async fn prepare(&self, query: &str) -> Result<Statement>;
    async fn query<'b>(&self, stmt: &Statement, params: Params<'b>) -> Result<Results>;
    async fn batch(&self, sql: &str) -> Result<()>;
    async fn execute_file(&self, path: &str) -> Result<()>;
    async fn transaction(&'a mut self) -> Result<GenericTransaction<'a>>;
}

#[async_trait]
impl<'a> ClientTrait<'a> for Client<'a> {
    async fn execute<'b>(&self, query: &str, params: Params<'b>) -> Result<u64> {
        Ok(self.connection.execute(query, params).await?)
    }
    async fn prepare(&self, query: &str) -> Result<Statement> {
        Ok(self.connection.prepare(query).await?)
    }
    async fn query<'b>(&self, stmt: &Statement, params: Params<'b>) -> Result<Results> {
        Ok(self.connection.query(stmt, params).await?)
    }
    async fn batch(&self, sql: &str) -> Result<()> {
        Ok(self.connection.batch_execute(sql).await?)
    }
    async fn execute_file(&self, path: &str) -> Result<()> {
        Ok(self.batch(&fs::read_to_string(path)?).await?)
    }
    async fn transaction(&'a mut self) -> Result<GenericTransaction<'a>> {
        Ok(Transaction::new(self.connection.transaction().await?).await?)
    }
}

impl<'a> Client<'a> {
    pub fn new(connection: PooledConnection<'a>) -> GenericClient {
        Box::new(Client { connection })
    }
}
