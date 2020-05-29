use async_trait::async_trait;
use crate::{model, error, repository, Result};
use mocking::Method;
use serde::export::PhantomData;

type MockResetRequest = Method<Option<model::PasswordResetRequest>, error::Error>;

#[derive(Clone)]
pub struct MockPasswordReset<T: model::Database> {
    phantom: PhantomData<T>,
    pub generate: MockResetRequest,
    pub by_id: MockResetRequest,
}

impl<T: model::Database> MockPasswordReset<T> {
    pub fn new() -> MockPasswordReset<T> {
        MockPasswordReset {
            phantom: PhantomData,
            generate: MockResetRequest::new("repository::PasswordResetRequest.generate()"),
            by_id: MockResetRequest::new("repository::PasswordResetRequest.by_id()"),
        }
    }
    pub async fn generate(&self, _email: &str) -> Result<Option<model::PasswordResetRequest>> {
        self.generate.call()
    }
    pub async fn by_id(&self, _id: &str) -> Result<Option<model::PasswordResetRequest>> {
        self.by_id.call()
    }
}

#[async_trait]
impl<T: model::Database> repository::PasswordResetRequest for MockPasswordReset<T> {
    async fn generate(&self, _email: &str) -> Result<Option<model::PasswordResetRequest>> {
        self.generate.call()
    }
    async fn by_id(&self, _id: &str) -> Result<Option<model::PasswordResetRequest>> {
        self.by_id.call()
    }
}