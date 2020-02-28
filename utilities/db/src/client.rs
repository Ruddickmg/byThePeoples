use crate::{connection, transaction, Error, Params};
use std::fs;
use tokio_postgres;

pub struct Client<'a> {
    connection: connection::Connection<'a>,
}

impl<'a> Client<'a> {
    pub fn new(client: connection::Connection<'a>) -> Client<'a> {
        Client { connection: client }
    }
    pub async fn execute(&mut self, query: &str, params: Params<'a>) -> Result<u64, Error> {
        Ok(self.connection.execute(query, params).await?)
    }
    pub async fn prepare(&mut self, query: &str) -> Result<tokio_postgres::Statement, Error> {
        Ok(self.connection.prepare(query).await?)
    }
    pub async fn query(
        &mut self,
        stmt: &tokio_postgres::Statement,
        params: Params<'a>,
    ) -> Result<Vec<tokio_postgres::Row>, Error> {
        Ok(self.connection.query(stmt, params).await?)
    }
    pub async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        Ok(self.connection.batch_execute(sql).await?)
    }
    pub async fn transaction(&'a mut self) -> Result<transaction::Transaction<'a>, Error> {
        Ok(transaction::Transaction::new(&mut self.connection).await?)
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        Ok(self.batch(&fs::read_to_string(path)?).await?)
    }
}
