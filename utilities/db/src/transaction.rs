use crate::{client, Params, Result, Results, Statement};
use async_trait::async_trait;
use std::fs;
use tokio_postgres;

pub struct Transaction<'a> {
    transaction: tokio_postgres::Transaction<'a>,
}

pub type GenericTransaction<'a> = Box<dyn TransactionTrait<'a> + 'a + Send + Sync>;

#[async_trait]
pub trait TransactionTrait<'a>: client::ClientTrait<'a> {
    async fn commit(self) -> Result<()>;
}

#[async_trait]
impl<'a> TransactionTrait<'a> for Transaction<'a> {
    async fn commit(self) -> Result<()> {
        Ok(self.transaction.commit().await?)
    }
}

#[async_trait]
impl<'a> client::ClientTrait<'a> for Transaction<'a> {
    async fn execute<'b>(&self, query: &str, params: Params<'b>) -> Result<u64> {
        Ok(self.transaction.execute(query, params).await?)
    }
    async fn prepare(&self, query: &str) -> Result<Statement> {
        Ok(self.transaction.prepare(query).await?)
    }
    async fn query<'b>(&self, stmt: &Statement, params: Params<'b>) -> Result<Results> {
        Ok(self.transaction.query(stmt, params).await?)
    }
    async fn batch(&self, sql: &str) -> Result<()> {
        Ok(self.transaction.batch_execute(sql).await?)
    }
    async fn execute_file(&self, path: &str) -> Result<()> {
        let sql = fs::read_to_string(path)?;
        Ok(self.batch(&sql).await?)
    }
    async fn transaction(&'a mut self) -> Result<GenericTransaction<'a>> {
        unimplemented!()
    }
}

impl<'a> Transaction<'a> {
    pub async fn new(
        transaction: tokio_postgres::Transaction<'a>,
    ) -> Result<GenericTransaction<'a>> {
        Ok(Box::new(Transaction { transaction }))
    }
}
