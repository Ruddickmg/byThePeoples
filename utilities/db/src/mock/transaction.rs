use super::Mocker;
use crate::{
    client,
    transaction::{GenericTransaction, TransactionTrait},
    Error, Params, Results, Statement,
};
use async_trait::async_trait;
use std::fs;
use tokio_postgres;

pub struct MockTransaction<'a> {
    commit: Mocker<(), (), Error>,
    execute: Mocker<u64, Params<'a>, Error>,
    prepare: Mocker<Statement, String, Error>,
    query: Mocker<Results, Params<'a>, Error>,
    batch: Mocker<(), String, Error>,
    execute_file: Mocker<(), String, Error>,
}

#[async_trait]
impl<'a> TransactionTrait<'a> for MockTransaction<'a> {
    async fn commit(&self) -> Result<(), Error> {
        self.commit().await
    }
}

#[async_trait]
impl<'b> client::ClientTrait<'b> for MockTransaction<'_> {
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
    async fn transaction(&'b mut self) -> Result<GenericTransaction<'b>, Error> {
        self.transaction().await
    }
    async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        self.execute_file(path).await
    }
}

impl<'a> MockTransaction<'a> {
    pub async fn new() -> Result<GenericTransaction<'a>, Error> {
        Ok(Box::new(MockTransaction {
            commit: Mocker::new("Transaction.commit()"),
            execute: Mocker::new("Transaction.execute()"),
            prepare: Mocker::new("Transaction.prepare()"),
            query: Mocker::new("Transaction.query()"),
            batch: Mocker::new("Transaction.batch()"),
            execute_file: Mocker::new("Transaction.execute_file()"),
        }))
    }
    pub async fn commit(self) -> Result<(), Error> {
        self.commit.call(None)
    }
    pub async fn prepare(self, query: &str) -> Result<Statement, Error> {
        self.prepare.call(Some(String::from(query)))
    }
    pub async fn query<'b>(self, _: &Statement, params: Params<'b>) -> Result<Results, Error> {
        self.query.call(Some(params))
    }
    pub async fn batch(&mut self, sql: &str) -> Result<(), Error> {
        self.batch.call(Some(String::from(sql)))
    }
    pub async fn execute_file(&mut self, path: &str) -> Result<(), Error> {
        self.execute_file.call(Some(String::from(path)))
    }
}
