use crate::{connection, Error, Params, Results, Statement};
use async_trait::async_trait;
use std::fs;
use tokio_postgres;

pub struct Transaction<'a> {
    transaction: tokio_postgres::Transaction<'a>,
}

pub type GenericTransaction<'a> = Box<dyn TransactionTrait<'a> + 'a>;

#[async_trait]
pub trait TransactionTrait<'a> {
    async fn commit(self) -> Result<(), Error>;
    async fn execute<'b>(&mut self, query: &str, params: Params<'b>) -> Result<u64, Error>;
    async fn prepare(&mut self, query: &str) -> Result<Statement, Error>;
    async fn query<'b>(&mut self, stmt: &Statement, params: Params<'b>) -> Result<Results, Error>;
    async fn batch(&mut self, sql: &str) -> Result<(), Error>;
    async fn execute_file(&mut self, path: &str) -> Result<(), Error>;
}

#[async_trait]
impl<'a> TransactionTrait<'a> for Transaction<'a> {
    async fn commit(self) -> Result<(), Error> {
        self.commit().await
    }
    async fn execute<'b>(&mut self, query: &str, params: Params<'b>) -> Result<u64, Error> {
        self.execute(query, params).await
    }
    async fn prepare(&mut self, query: &str) -> Result<Statement, Error> {
        self.prepare(query).await
    }
    async fn query<'b>(&mut self, stmt: &Statement, params: Params<'b>) -> Result<Results, Error> {
        self.query(stmt, params).await
    }
    async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        self.batch(sql).await
    }
    async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        self.execute_file(path).await
    }
}

impl<'a> Transaction<'a> {
    pub async fn new(
        connection: &'a mut connection::Connection<'a>,
    ) -> Result<Transaction<'a>, Error> {
        Ok(Transaction {
            transaction: connection.transaction().await?,
        })
    }
    pub async fn commit(self) -> Result<(), Error> {
        Ok(self.transaction.commit().await?)
    }
    pub async fn execute<'b>(&mut self, query: &str, params: Params<'b>) -> Result<u64, Error> {
        Ok(self.transaction.execute(query, params).await?)
    }
    pub async fn prepare(&mut self, query: &str) -> Result<Statement, Error> {
        Ok(self.transaction.prepare(query).await?)
    }
    pub async fn query<'b>(
        &mut self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Results, Error> {
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
