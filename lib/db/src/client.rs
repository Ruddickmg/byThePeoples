use crate::{transaction::Transaction, Params, PooledConnection, Result, Row, Statement};
use async_trait::async_trait;
use std::{fs, marker::Send};

pub struct Client<'a> {
    connection: PooledConnection<'a>,
}

impl<'a: 'c, 'c> Client<'a> {
    pub fn new(connection: PooledConnection<'a>) -> Client {
        Client { connection }
    }
    pub async fn execute<'b>(&self, query: &str, params: Params<'b>) -> Result<u64> {
        Ok(self.connection.execute(query, params).await?)
    }
    pub async fn prepare(&self, query: &str) -> Result<Statement> {
        Ok(self.connection.prepare(query).await?)
    }
    pub async fn query<'b, T: Send + From<Row>>(
        &self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Vec<T>> {
        let mut result: Vec<T> = vec![];
        let query_results = self.connection.query(stmt, params).await?;
        for row in query_results {
            result.push(T::from(row));
        }
        Ok(result)
    }
    pub async fn batch(&self, sql: &str) -> Result<()> {
        Ok(self.connection.batch_execute(sql).await?)
    }
    pub async fn execute_file(&self, path: &str) -> Result<()> {
        Ok(self.batch(&fs::read_to_string(path)?).await?)
    }
    pub async fn transaction(&'c mut self) -> Result<Transaction<'c>> {
        Ok(Transaction::new(self.connection.transaction().await?).await?)
    }
}

#[async_trait]
pub trait ClientTrait<'a: 'c, 'c> {
    async fn execute<'b>(&self, query: &str, params: Params<'b>) -> Result<u64>;
    async fn prepare(&self, query: &str) -> Result<Statement>;
    async fn query<'b, T: Send + From<Row>>(
        &self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Vec<T>>;
    async fn batch(&self, sql: &str) -> Result<()>;
    async fn execute_file(&self, path: &str) -> Result<()>;
    async fn transaction(&'c mut self) -> Result<Transaction<'c>>;
}

#[async_trait]
impl<'a: 'c, 'c> ClientTrait<'a, 'c> for Client<'a> {
    async fn execute<'b>(&self, query: &str, params: Params<'b>) -> Result<u64> {
        Ok(self.connection.execute(query, params).await?)
    }
    async fn prepare(&self, query: &str) -> Result<Statement> {
        Ok(self.connection.prepare(query).await?)
    }
    async fn query<'b, T: Send + From<Row>>(
        &self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Vec<T>> {
        let mut result: Vec<T> = vec![];
        let query_results = self.connection.query(stmt, params).await?;
        for row in query_results {
            result.push(T::from(row));
        }
        Ok(result)
    }
    async fn batch(&self, sql: &str) -> Result<()> {
        Ok(self.connection.batch_execute(sql).await?)
    }
    async fn execute_file(&self, path: &str) -> Result<()> {
        Ok(self.batch(&fs::read_to_string(path)?).await?)
    }
    async fn transaction(&'c mut self) -> Result<Transaction<'c>> {
        Ok(Transaction::new(self.connection.transaction().await?).await?)
    }
}
