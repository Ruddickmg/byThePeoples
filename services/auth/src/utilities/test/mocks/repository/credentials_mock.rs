use crate::{model, repository, error::Error, Result};
use async_trait::async_trait;
use serde::export::PhantomData;

type CredentialResults = Result<Option<model::Credentials>>;
type MockedOptionCredentials = mocking::Method<Option<model::Credentials>, Error>;
type MockedStatusResult = mocking::Method<repository::CredentialStatus, Error>;
type MockedCountResult = mocking::Method<i32, Error>;
type MockedCredentials = mocking::Method<model::Credentials, Error>;

#[derive(Clone)]
pub struct MockCredentials<T: model::Database> {
    pub by_name: MockedOptionCredentials,
    pub by_email: MockedOptionCredentials,
    pub by_id: MockedOptionCredentials,
    pub get_status: MockedStatusResult,
    pub update_credentials: MockedCredentials,
    pub update_password_hash: MockedCredentials,
    pub save_credentials: MockedCredentials,
    pub mark_as_deleted_by_email: MockedCountResult,
    phantom: PhantomData<T>,
}

impl<T: model::Database> MockCredentials<T> {
    pub fn new() -> MockCredentials<T> {
        MockCredentials {
            by_name: MockedOptionCredentials::new("repository::Credentials.by_name()"),
            by_email: MockedOptionCredentials::new("repository::Credentials.by_email()"),
            by_id: MockedOptionCredentials::new("repository::Credentials.by_id()"),
            get_status: MockedStatusResult::new("repository::Credentials.get_status()"),
            update_credentials: MockedCredentials::new(
                "repository::Credentials.update_credentials()",
            ),
            update_password_hash: MockedCredentials::new("repository::Credentials.update_password_hash()"),
            save_credentials: MockedCredentials::new("repository::Credentials.save_credentials()"),
            mark_as_deleted_by_email: MockedCountResult::new(
                "repository::Credentials.mark_as_deleted_by_email()",
            ),
            phantom: PhantomData,
        }
    }
    pub async fn by_name(&self, _name: &str) -> CredentialResults {
        self.by_name.call()
    }
    pub async fn by_email(&self, _email: &str) -> CredentialResults {
        self.by_email.call()
    }
    pub async fn by_id(&self, _id: i32) -> CredentialResults {
        self.by_id.call()
    }
    pub async fn get_status(
        &self,
        _credentials: &model::FullRequest,
    ) -> Result<repository::CredentialStatus> {
        self.get_status.call()
    }
    pub async fn update_credentials(
        &self,
        _credentials: &model::Credentials,
    ) -> Result<model::Credentials> {
        self.update_credentials.call()
    }
    pub async fn update_password_hash(&self, _id: &i32, _hash: &str) -> Result<model::Credentials> {
        self.update_password_hash.call()
    }
    pub async fn save_credentials(
        &self,
        _name: &str,
        _email: &str,
    ) -> Result<model::Credentials> {
        self.save_credentials.call()
    }
    pub async fn mark_as_deleted_by_email(&mut self, _email: &str) -> Result<i32> {
        self.mark_as_deleted_by_email.call()
    }
}

#[async_trait]
impl<T: model::Database> repository::Credentials for MockCredentials<T> {
    async fn by_name(&self, _name: &str) -> CredentialResults {
        self.by_name.call()
    }
    async fn by_email(&self, _email: &str) -> CredentialResults {
        self.by_email.call()
    }
    async fn by_id(&self, _id: i32) -> CredentialResults {
        self.by_id.call()
    }
    async fn get_status(&self, _name: &str, _email: &str) -> Result<repository::CredentialStatus> {
        self.get_status.call()
    }
    async fn update_credentials(
        &self,
        _credentials: &model::Credentials,
    ) -> Result<model::Credentials> {
        self.update_credentials.call()
    }
    async fn update_password_hash(&self, _id: &i32, _hash: &str) -> Result<model::Credentials> {
        self.update_password_hash.call()
    }
    async fn save_credentials(
        &self,
        _credentials: &model::FullRequest,
    ) -> Result<model::Credentials> {
        self.save_credentials.call()
    }
    async fn mark_as_deleted_by_email(&self, _email: &str) -> Result<i32> {
        self.mark_as_deleted_by_email.call()
    }
}
