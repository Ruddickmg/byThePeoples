use crate::{model, model::credentials, repository, Error};
use async_trait::async_trait;

type CredentialResults = Result<Option<model::Credentials>, Error>;
type MockedOptionCredentials = mocking::Method<Option<model::Credentials>, Error>;
type MockStatusResult = mocking::Method<repository::Status, Error>;
type MockedCountResult = mocking::Method<i32, Error>;
type MockedCredentials = mocking::Method<model::Credentials, Error>;

pub struct MockCredentials {
    by_name: MockedOptionCredentials,
    by_email: MockedOptionCredentials,
    get_status: MockStatusResult,
    update_credentials: MockedCredentials,
    save_credentials: MockedCountResult,
    mark_as_deleted_by_email: MockedCountResult,
}

impl MockCredentials {
    pub fn new() -> MockCredentials {
        MockCredentials {
            by_name: MockedOptionCredentials::new("repository::Credentials.by_name()"),
            by_email: MockedOptionCredentials::new("repository::Credentials.by_email()"),
            get_status: MockStatusResult::new("repository::Credentials.get_status()"),
            update_credentials: MockedCredentials::new(
                "repository::Credentials.update_credentials()",
            ),
            save_credentials: MockedCountResult::new("repository::Credentials.save_credentials()"),
            mark_as_deleted_by_email: MockedCountResult::new(
                "repository::Credentials.mark_as_deleted_by_email()",
            ),
        }
    }
    pub async fn by_name(&mut self, _name: &str) -> CredentialResults {
        self.by_name.call()
    }
    pub async fn by_email(&mut self, _email: &str) -> CredentialResults {
        self.by_email.call()
    }
    pub async fn get_status(
        &mut self,
        _credentials: &model::FullRequest,
    ) -> Result<repository::Status, Error> {
        self.get_status.call()
    }
    pub async fn update_credentials(
        &mut self,
        _credentials: &model::Credentials,
    ) -> Result<model::Credentials, Error> {
        self.update_credentials.call()
    }
    pub async fn save_credentials(
        &mut self,
        _credentials: &model::FullRequest,
    ) -> Result<i32, Error> {
        self.save_credentials.call()
    }
    pub async fn mark_as_deleted_by_email(&mut self, _email: &str) -> Result<i32, Error> {
        self.mark_as_deleted_by_email.call()
    }
}

#[async_trait]
impl<T: model::Database> repository::Credentials<T> for MockCredentials {
    async fn by_name(&mut self, _name: &str) -> CredentialResults {
        self.by_name.call()
    }
    async fn by_email(&mut self, _email: &str) -> CredentialResults {
        self.by_email.call()
    }
    async fn get_status(
        &mut self,
        _credentials: &model::FullRequest,
    ) -> Result<repository::Status, Error> {
        self.get_status.call()
    }
    async fn update_credentials(
        &mut self,
        _credentials: &model::Credentials,
    ) -> Result<model::Credentials, Error> {
        self.update_credentials.call()
    }
    async fn save_credentials(&mut self, _credentials: &model::FullRequest) -> Result<i32, Error> {
        self.save_credentials.call()
    }
    async fn mark_as_deleted_by_email(&mut self, _email: &str) -> Result<i32, Error> {
        self.mark_as_deleted_by_email.call()
    }
}
