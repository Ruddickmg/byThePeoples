use crate::connection;
use std::{fs, marker};
use tokio_postgres;

pub struct Transaction<'a> {
    transaction: tokio_postgres::Transaction<'a>,
}

impl<'a> Transaction<'a> {
    pub async fn new(
        connection: &'a mut connection::Connection<'a>,
    ) -> Result<Transaction<'a>, tokio_postgres::Error> {
        Ok(Transaction {
            transaction: connection.transaction().await?,
        })
    }
    pub async fn commit(self) -> Result<(), tokio_postgres::Error> {
        self.transaction.commit().await
    }
    pub async fn execute(
        &mut self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + marker::Sync)],
    ) -> Result<u64, tokio_postgres::Error> {
        self.transaction.execute(query, params).await
    }
    pub async fn prepare(
        &mut self,
        query: &str,
    ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
        self.transaction.prepare(query).await
    }
    pub async fn batch(&mut self, sql: &str) -> Result<(), tokio_postgres::Error> {
        self.transaction.batch_execute(sql).await
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<(), tokio_postgres::Error> {
        if let Ok(sql) = fs::read_to_string(path) {
            self.batch(&sql).await
        } else {
            panic!(format!("Could not get file contents from {}", path));
        }
    }
}
