use crate::{connection::Connection, transaction, Error, Params, Results, Statement};
use std::fs;

pub struct Client<'a> {
    connection: Connection<'a>,
}

impl<'a> Client<'a> {
    pub fn new(connection: Connection<'a>) -> Client<'a> {
        Client { connection }
    }
    pub async fn execute<'b>(&mut self, query: &str, params: Params<'b>) -> Result<u64, Error> {
        Ok(self.connection.execute(query, params).await?)
    }
    pub async fn prepare(&mut self, query: &str) -> Result<Statement, Error> {
        Ok(self.connection.prepare(query).await?)
    }
    pub async fn query<'b>(
        &mut self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Results, Error> {
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
