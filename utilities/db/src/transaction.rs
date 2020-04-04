use crate::{Params, Result, Row, Statement};
use std::fs;
use tokio_postgres;

pub struct Transaction<'a> {
    transaction: tokio_postgres::Transaction<'a>,
}

impl<'a> Transaction<'a> {
    async fn execute<'b>(&self, query: &str, params: Params<'b>) -> Result<u64> {
        Ok(self.transaction.execute(query, params).await?)
    }
    async fn prepare(&self, query: &str) -> Result<Statement> {
        Ok(self.transaction.prepare(query).await?)
    }
    async fn query<'b, T: From<Row>>(
        &self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Vec<T>> {
        let mut result: Vec<T> = vec![];
        let query_results = self.transaction.query(stmt, params).await?;
        for row in query_results {
            result.push(T::from(row));
        }
        Ok(result)
    }
    async fn batch(&self, sql: &str) -> Result<()> {
        Ok(self.transaction.batch_execute(sql).await?)
    }
    async fn execute_file(&self, path: &str) -> Result<()> {
        Ok(self.batch(&fs::read_to_string(path)?).await?)
    }
}

impl<'a> Transaction<'a> {
    pub async fn new(transaction: tokio_postgres::Transaction<'a>) -> Result<Transaction<'a>> {
        Ok(Transaction { transaction })
    }
}
