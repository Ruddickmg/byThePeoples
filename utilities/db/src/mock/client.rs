use super::Mocker;
use crate::{
    client::{ClientTrait, GenericClient},
    transaction::GenericTransaction,
    Error, Params, PooledConnection, Results, Statement, Transaction,
};
use async_trait::async_trait;

pub struct MockClient<'a> {
    pub execute: Mocker<u64, Params<'a>, Error>,
    pub prepare: Mocker<Statement, String, Error>,
    pub query: Mocker<Results, Params<'a>, Error>,
    pub batch: Mocker<(), String, Error>,
    pub transaction: Mocker<Transaction<'a>, (), Error>,
    pub execute_file: Mocker<(), String, Error>,
}

#[async_trait]
impl<'b> ClientTrait<'b> for MockClient<'b> {
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

    async fn transaction(&'b mut self) -> Result<GenericTransaction<'_>, Error> {
        self.transaction().await
    }

    async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        self.execute_file(path).await
    }
}

impl<'b> MockClient<'b> {
    pub fn new() -> GenericClient<'b> {
        Box::new(MockClient {
            execute: Mocker::new("execute"),
            prepare: Mocker::new("prepare"),
            query: Mocker::new("query"),
            batch: Mocker::new("batch"),
            transaction: Mocker::new("transaction"),
            execute_file: Mocker::new("execute_file"),
        })
    }
    pub async fn execute<'a>(self, _: &str, params: Params<'a>) -> Result<u64, Error> {
        self.execute.call(Some(params))
    }
    pub async fn prepare(self, query: &str) -> Result<Statement, Error> {
        self.prepare.call(Some(String::from(query)))
    }
    pub async fn query<'a>(self, query: &Statement, params: Params<'a>) -> Result<Results, Error> {
        self.query.call(Some(params))
    }
    pub async fn transaction(&'b mut self) -> Result<GenericTransaction<'_>, Error> {
        self.transaction.call(None)
    }
    pub async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        self.batch.call(Some(String::from(sql)))
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        self.execute_file.call(Some(String::from(path)))
    }
}
