use crate::{client, Error, Params, Results, Statement};
use async_trait::async_trait;
use std::fs;
use tokio_postgres;

pub struct Transaction<'a> {
    transaction: tokio_postgres::Transaction<'a>,
}

pub type GenericTransaction<'a> = Box<dyn TransactionTrait<'a> + 'a + Send + Sync>;

#[async_trait]
pub trait TransactionTrait<'a>: client::ClientTrait<'a> {
    async fn commit(&self) -> Result<(), Error>;
}

#[async_trait]
impl<'a> TransactionTrait<'a> for Transaction<'a> {
    async fn commit(&self) -> Result<(), Error> {
        self.commit().await
    }
}

#[async_trait]
impl<'b> client::ClientTrait<'b> for Transaction<'_> {
    async fn execute<'a>(&self, query: &str, params: Params<'a>) -> Result<u64, Error> {
        self.execute(query, params).await
    }
    async fn prepare(&self, query: &str) -> Result<Statement, Error> {
        self.prepare(query).await
    }
    async fn query<'a>(&self, query: &Statement, params: Params<'a>) -> Result<Results, Error> {
        self.query(query, params).await
    }
    async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        self.batch(sql).await
    }
    async fn transaction(&'b mut self) -> Result<GenericTransaction<'b>, Error> {
        self.transaction().await
    }
    async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        self.execute_file(path).await
    }
}

impl<'a> Transaction<'a> {
    pub async fn new(
        transaction: tokio_postgres::Transaction<'a>,
    ) -> Result<GenericTransaction<'a>, Error> {
        Ok(Box::new(Transaction { transaction }))
    }
    pub async fn commit(self) -> Result<(), Error> {
        Ok(self.transaction.commit().await?)
    }
    pub async fn prepare(self, query: &str) -> Result<Statement, Error> {
        Ok(self.transaction.prepare(query).await?)
    }
    pub async fn query<'b>(self, stmt: &Statement, params: Params<'b>) -> Result<Results, Error> {
        Ok(self.transaction.query(stmt, params).await?)
    }
    pub async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        Ok(self.transaction.batch_execute(sql).await?)
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        let sql = fs::read_to_string(path)?;
        Ok(self.batch(&sql).await?)
    }
}
