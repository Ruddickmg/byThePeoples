use crate::{connection::Connection, transaction, Error, Params, Results, Statement};
use async_trait::async_trait;
use std::fs;

pub struct Client<'a> {
    connection: Connection<'a>,
}

pub type GenericClient<'a> = Box<dyn ClientTrait<'a> + 'a>;

#[async_trait]
pub trait ClientTrait<'a> {
    async fn execute<'b>(&'a mut self, query: &str, params: Params<'b>) -> Result<u64, Error>;
    async fn prepare(&'a mut self, query: &str) -> Result<Statement, Error>;
    async fn query<'b>(
        &'a mut self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Results, Error>;
    async fn batch(&mut self, sql: &str) -> Result<(), Error>;
    async fn transaction(&'a mut self) -> Result<transaction::GenericTransaction<'a>, Error>;
    async fn execute_file(&mut self, path: &str) -> Result<(), Error>;
}

#[async_trait]
impl<'a> ClientTrait<'a> for Client<'a> {
    async fn execute<'b>(&'a mut self, query: &str, params: Params<'b>) -> Result<u64, Error> {
        self.execute(query, params).await
    }
    async fn prepare(&'a mut self, query: &str) -> Result<Statement, Error> {
        self.prepare(query).await
    }
    async fn query<'b>(
        &'a mut self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Results, Error> {
        self.query(stmt, params).await
    }
    async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        self.batch(sql).await
    }
    async fn transaction(&'a mut self) -> Result<transaction::GenericTransaction<'a>, Error> {
        self.transaction().await
    }
    async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        self.execute_file(path).await
    }
}

impl<'a> Client<'a> {
    pub fn new(connection: Connection<'a>) -> Client<'a> {
        Client { connection }
    }
    fn get_connection(&'a mut self) -> &'a mut Connection<'a> {
        &mut self.connection
    }
    pub async fn execute<'b>(&'a mut self, query: &str, params: Params<'b>) -> Result<u64, Error> {
        Ok(self.get_connection().execute(query, params).await?)
    }
    pub async fn prepare(&'a mut self, query: &str) -> Result<Statement, Error> {
        Ok(self.get_connection().prepare(query).await?)
    }
    pub async fn query<'b>(
        &'a mut self,
        stmt: &Statement,
        params: Params<'b>,
    ) -> Result<Results, Error> {
        Ok(self.get_connection().query(stmt, params).await?)
    }
    pub async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        Ok(self.connection.batch_execute(sql).await?)
    }
    pub async fn transaction(&'a mut self) -> Result<transaction::GenericTransaction<'a>, Error> {
        Ok(Box::new(
            transaction::Transaction::new(&mut self.connection).await?,
        ))
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        Ok(self.batch(&fs::read_to_string(path)?).await?)
    }
}
