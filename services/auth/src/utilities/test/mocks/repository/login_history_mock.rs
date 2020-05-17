use crate::{model, repository, Error};
use async_trait::async_trait;
use serde::export::PhantomData;

type MockFailedLoginResponse = mocking::Method<model::FailedLogin, Error>;
type MockEmptyResponse = mocking::Method<(), Error>;

#[derive(Clone)]
pub struct MockLoginHistory<T: model::Database> {
    phantom: PhantomData<T>,
    pub log: MockFailedLoginResponse,
    pub get: MockFailedLoginResponse,
    pub delete: MockEmptyResponse,
    pub suspend: MockEmptyResponse,
}

impl<T: model::Database> MockLoginHistory<T> {
    pub fn new() -> MockLoginHistory<T> {
        MockLoginHistory {
            phantom: PhantomData,
            log: MockFailedLoginResponse::new("Repository::LoginHistory.log()"),
            get: MockFailedLoginResponse::new("Repository::LoginHistory.get()"),
            delete: MockEmptyResponse::new("Repository::LoginHistory.delete()"),
            suspend: MockEmptyResponse::new("Repository::LoginHistory.suspend()"),
        }
    }
    pub async fn log(&self, _id: &model::CredentialId) -> Result<model::FailedLogin, Error> {
        self.log.call()
    }
    pub async fn get(&self, _id: &model::CredentialId) -> Result<model::FailedLogin, Error> {
        self.get.call()
    }
    pub async fn delete(&self, _id: &model::CredentialId) -> Result<(), Error> {
        self.delete.call()
    }
    pub async fn suspend(&self, _user_id: &model::CredentialId) -> Result<(), Error> {
        self.suspend.call()
    }
}

#[async_trait]
impl<T: model::Database> repository::LoginHistory<T> for MockLoginHistory<T> {
    async fn log(&self, _id: &model::CredentialId) -> Result<model::FailedLogin, Error> {
        self.log.call()
    }
    async fn get(&self, _id: &model::CredentialId) -> Result<model::FailedLogin, Error> {
        self.get.call()
    }
    async fn delete(&self, _id: &model::CredentialId) -> Result<(), Error> {
        self.delete.call()
    }
    async fn suspend(&self, _user_id: &model::CredentialId) -> Result<(), Error> {
        self.suspend.call()
    }
}
