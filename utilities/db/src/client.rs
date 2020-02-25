use crate::{connection, transaction};
use std::{fs, marker};
use tokio_postgres;

pub struct Client<'a> {
    connection: connection::Client<'a>,
}

impl<'a> Client<'a> {
    pub fn new(client: connection::Client<'a>) -> Client<'a> {
        Client { connection: client }
    }
    pub async fn execute(
        &mut self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + marker::Sync)],
    ) -> Result<u64, tokio_postgres::Error> {
        self.connection.execute(query, params).await
    }
    pub async fn prepare(
        &mut self,
        query: &str,
    ) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {
        self.connection.prepare(query).await
    }
    pub async fn batch(&mut self, sql: &str) -> Result<(), tokio_postgres::Error> {
        self.connection.batch_execute(sql).await
    }
    pub async fn transaction(
        &'a mut self,
    ) -> Result<transaction::Transaction<'a>, tokio_postgres::Error> {
        transaction::Transaction::new(&mut self.connection).await
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<(), tokio_postgres::Error> {
        if let Ok(sql) = fs::read_to_string(path) {
            self.batch(&sql).await
        } else {
            panic!(format!("Could not get file contents from {}", path));
        }
    }
}
