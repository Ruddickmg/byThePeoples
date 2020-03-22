use super::MockMethod;
use crate::{
    client,
    transaction::{GenericTransaction, TransactionTrait},
    Params, Result, Results, Statement,
};
use async_trait::async_trait;

pub struct MockTransaction {
    commit: MockMethod<()>,
    execute: MockMethod<u64>,
    prepare: MockMethod<Statement>,
    query: MockMethod<Results>,
    batch: MockMethod<()>,
    execute_file: MockMethod<()>,
}

#[async_trait]
impl<'a> TransactionTrait<'a> for MockTransaction {
    async fn commit(&self) -> Result<()> {
        self.commit().await
    }
}

#[async_trait]
impl<'b> client::ClientTrait for MockTransaction {
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
    async fn transaction(&'b mut self) -> Result<GenericTransaction<'b>> {
        self.transaction().await
    }
    async fn execute_file(&mut self, path: &str) -> Result<()> {
        self.execute_file(path).await
    }
}

impl<'a> MockTransaction {
    pub async fn new() -> Result<GenericTransaction<'a>> {
        Ok(Box::new(MockTransaction {
            commit: mock::Method::new("Transaction.commit()"),
            execute: mock::Method::new("Transaction.execute()"),
            prepare: mock::Method::new("Transaction.prepare()"),
            query: mock::Method::new("Transaction.query()"),
            batch: mock::Method::new("Transaction.batch()"),
            execute_file: mock::Method::new("Transaction.execute_file()"),
        }))
    }
    pub async fn commit(&mut self) -> Result<()> {
        self.commit.call()
    }
    pub async fn prepare(&mut self, _: &str) -> Result<Statement> {
        self.prepare.call()
    }
    pub async fn query<'b>(&mut self, _: &Statement, _: Params<'b>) -> Result<Results> {
        self.query.call()
    }
    pub async fn batch(&mut self, _: &str) -> Result<()> {
        self.batch.call()
    }
    pub async fn execute_file(&mut self, _: &str) -> Result<()> {
        self.execute_file.call()
    }
}
