use super::MockMethod;
use crate::{
    client::{ClientTrait, GenericClient},
    transaction::GenericTransaction,
    Params, Result, Results, Statement, Transaction,
};
use async_trait::async_trait;

pub struct MockClient<'a> {
    pub execute: MockMethod<u64>,
    pub prepare: MockMethod<Statement>,
    pub query: MockMethod<Results>,
    pub batch: MockMethod<()>,
    pub transaction: MockMethod<Transaction<'a>>,
    pub execute_file: MockMethod<()>,
}

#[async_trait]
impl<'b> ClientTrait<'b> for MockClient<'b> {
    async fn execute<'a>(&self, query: &str, params: Params<'a>) -> Result<u64> {
        self.execute(query, params).await
    }
    async fn prepare(&self, query: &str) -> Result<Statement> {
        self.prepare(query).await
    }

    async fn query<'a>(&self, query: &Statement, params: Params<'a>) -> Result<Results> {
        self.query(query, params).await
    }

    async fn batch(&mut self, sql: &str) -> Result<()> {
        self.batch(sql).await
    }

    async fn transaction(&'b mut self) -> Result<GenericTransaction<'_>> {
        self.transaction().await
    }

    async fn execute_file(&mut self, path: &str) -> Result<()> {
        self.execute_file(path).await
    }
}

impl<'b> MockClient<'b> {
    pub fn new() -> GenericClient<'b> {
        Box::new(MockClient {
            execute: mock::Method::new("execute"),
            prepare: mock::Method::new("prepare"),
            query: mock::Method::new("query"),
            batch: mock::Method::new("batch"),
            transaction: mock::Method::new("transaction"),
            execute_file: mock::Method::new("execute_file"),
        })
    }
    pub async fn execute<'a>(&mut self, _: &str, _: Params<'a>) -> Result<u64> {
        Ok(self.execute.call()?)
    }
    pub async fn prepare(&mut self, _: &str) -> Result<Statement> {
        self.prepare.call()
    }
    pub async fn query<'a>(&mut self, _: &Statement, _: Params<'a>) -> Result<Results> {
        self.query.call()
    }
    pub async fn transaction(&'b mut self) -> Result<GenericTransaction<'b>> {
        self.transaction.call()
    }
    pub async fn batch(&mut self, _: &str) -> Result<()> {
        self.batch.call()
    }
    pub async fn execute_file(&mut self, _: &str) -> Result<()> {
        self.execute_file.call()
    }
}
