use crate::{transaction::Transaction, Params, PooledConnection, Result, Row, Statement};
use std::fs;

pub struct Client<'a> {
    connection: PooledConnection<'a>,
}

impl<'a> Client<'a> {
    pub async fn execute<'b>(&self, query: &str, params: Params<'b>) -> Result<u64> {
        Ok(self.connection.execute(query, params).await?)
    }
    pub async fn prepare(&self, query: &str) -> Result<Statement> {
        Ok(self.connection.prepare(query).await?)
    }
    pub async fn query<'b, T: From<Row>>(
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
    pub async fn transaction(&'a mut self) -> Result<Transaction<'a>> {
        Ok(Transaction::new(self.connection.transaction().await?).await?)
    }
}

impl<'a> Client<'a> {
    pub fn new(connection: PooledConnection<'a>) -> Client {
        Client { connection }
    }
}
