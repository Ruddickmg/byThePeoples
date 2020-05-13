use crate::{model, model::credentials, repository, Error};
use async_trait::async_trait;
use serde::export::PhantomData;

type CredentialResults = Result<Option<model::Credentials>, Error>;
type MockedOptionCredentials = mocking::Method<Option<model::Credentials>, Error>;
type MockedStatusResult = mocking::Method<repository::Status, Error>;
type MockedCountResult = mocking::Method<i32, Error>;
type MockedCredentials = mocking::Method<model::Credentials, Error>;

#[derive(Clone)]
pub struct MockCredentials<T: model::Database> {
    by_name: MockedOptionCredentials,
    by_email: MockedOptionCredentials,
    get_status: MockedStatusResult,
    update_credentials: MockedCredentials,
    save_credentials: MockedCountResult,
    mark_as_deleted_by_email: MockedCountResult,
    phantom: PhantomData<T>,
}

impl<T: model::Database> MockCredentials<T> {
    pub fn new() -> MockCredentials<T> {
        MockCredentials {
            by_name: MockedOptionCredentials::new("repository::Credentials.by_name()"),
            by_email: MockedOptionCredentials::new("repository::Credentials.by_email()"),
            get_status: MockedStatusResult::new("repository::Credentials.get_status()"),
            update_credentials: MockedCredentials::new(
                "repository::Credentials.update_credentials()",
            ),
            save_credentials: MockedCountResult::new("repository::Credentials.save_credentials()"),
            mark_as_deleted_by_email: MockedCountResult::new(
                "repository::Credentials.mark_as_deleted_by_email()",
            ),
            phantom: PhantomData,
        }
    }
    pub async fn by_name(&self, _name: &str) -> CredentialResults {
        self.by_name.call_ref()
    }
    pub async fn by_email(&self, _email: &str) -> CredentialResults {
        self.by_email.call_ref()
    }
    pub async fn get_status(
        &self,
        _credentials: &model::FullRequest,
    ) -> Result<repository::Status, Error> {
        self.get_status.call_ref()
    }
    pub async fn update_credentials(
        &self,
        _credentials: &model::Credentials,
    ) -> Result<model::Credentials, Error> {
        self.update_credentials.call_ref()
    }
    pub async fn save_credentials(&self, _credentials: &model::FullRequest) -> Result<i32, Error> {
        self.save_credentials.call_ref()
    }
    pub async fn mark_as_deleted_by_email(&mut self, _email: &str) -> Result<i32, Error> {
        self.mark_as_deleted_by_email.call_ref()
    }
}

#[async_trait]
impl<T: model::Database> repository::Credentials<T> for MockCredentials<T> {
    async fn by_name(&self, _name: &str) -> CredentialResults {
        self.by_name.call_ref()
    }
    async fn by_email(&self, _email: &str) -> CredentialResults {
        self.by_email.call_ref()
    }
    async fn get_status(
        &self,
        _credentials: &model::FullRequest,
    ) -> Result<repository::Status, Error> {
        self.get_status.call_ref()
    }
    async fn update_credentials(
        &self,
        _credentials: &model::Credentials,
    ) -> Result<model::Credentials, Error> {
        self.update_credentials.call_ref()
    }
    async fn save_credentials(&self, _credentials: &model::FullRequest) -> Result<i32, Error> {
        self.save_credentials.call_ref()
    }
    async fn mark_as_deleted_by_email(&self, _email: &str) -> Result<i32, Error> {
        self.mark_as_deleted_by_email.call_ref()
    }
}
