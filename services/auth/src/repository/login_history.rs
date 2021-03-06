use crate::{model, model::credentials, Result};
use async_trait::async_trait;
use futures::future::join;
use std::marker::{Send, Sync};

const GET_FAILED_LOGIN: &str =
    "SELECT user_id, attempts, created_at, updated_at FROM auth.failed_login WHERE user_id = $1";
const CREATE_OR_UPDATE_FAILED_LOGIN: &str = "INSERT INTO auth.failed_login(user_id) VALUES ($1)
 ON CONFLICT (user_id) DO
     UPDATE
     SET
      attempts = failed_login.attempts + 1,
      updated_at = CURRENT_TIMESTAMP
RETURNING user_id, attempts, created_at, updated_at;";
const DELETE_FAILED_LOGIN_RECORD: &str = "DELETE FROM auth.failed_login WHERE user_id = $1";

pub type AppLoginHistory = LoginHistoryRepository<model::DatabaseConnection>;

#[derive(Clone)]
pub struct LoginHistoryRepository<T: model::Database> {
    db: T,
}

impl<T: model::Database> LoginHistoryRepository<T> {
    pub fn new(db: T) -> LoginHistoryRepository<T> {
        LoginHistoryRepository { db }
    }
    pub async fn log(&self, id: &model::CredentialId) -> Result<model::FailedLogin> {
        let client = self.db.client().await?;
        let stmt = client.prepare(CREATE_OR_UPDATE_FAILED_LOGIN).await?;
        Ok(client
            .query::<model::FailedLogin>(&stmt, &[&id])
            .await?
            .remove(0))
    }
    pub async fn get(&self, id: &model::CredentialId) -> Result<model::FailedLogin> {
        let client = self.db.client().await?;
        let stmt = client.prepare(GET_FAILED_LOGIN).await?;
        Ok(client
            .query::<model::FailedLogin>(&stmt, &[&id])
            .await?
            .remove(0))
    }
    pub async fn delete(&self, id: &model::CredentialId) -> Result<()> {
        self.db
            .client()
            .await?
            .execute(DELETE_FAILED_LOGIN_RECORD, &[&id])
            .await?;
        Ok(())
    }
    pub async fn suspend(&self, user_id: &model::CredentialId) -> Result<()> {
        let failed_logins = self.log(user_id).await?;
        if failed_logins.exceeded_limit() {
            let reset = self.delete(user_id);
            if failed_logins.expired()? {
                reset.await?;
            } else {
                let (..) = join(
                    reset,
                    self.db
                        .client()
                        .await?
                        .execute(credentials::query::SUSPEND, &[&user_id]),
                )
                .await;
            }
        }
        Ok(())
    }
}

#[async_trait]
pub trait LoginHistory: Clone + Send + Sync {
    async fn log(&self, id: &model::CredentialId) -> Result<model::FailedLogin>;
    async fn get(&self, id: &model::CredentialId) -> Result<model::FailedLogin>;
    async fn delete(&self, id: &model::CredentialId) -> Result<()>;
    async fn suspend(&self, user_id: &model::CredentialId) -> Result<()>;
}

#[async_trait]
impl<T: model::Database> LoginHistory for LoginHistoryRepository<T> {
    async fn log(&self, id: &model::CredentialId) -> Result<model::FailedLogin> {
        let client = self.db.client().await?;
        let stmt = client.prepare(CREATE_OR_UPDATE_FAILED_LOGIN).await?;
        Ok(client
            .query::<model::FailedLogin>(&stmt, &[&id])
            .await?
            .remove(0))
    }
    async fn get(&self, id: &model::CredentialId) -> Result<model::FailedLogin> {
        let client = self.db.client().await?;
        let stmt = client.prepare(GET_FAILED_LOGIN).await?;
        Ok(client
            .query::<model::FailedLogin>(&stmt, &[&id])
            .await?
            .remove(0))
    }
    async fn delete(&self, id: &model::CredentialId) -> Result<()> {
        self.db
            .client()
            .await?
            .execute(DELETE_FAILED_LOGIN_RECORD, &[&id])
            .await?;
        Ok(())
    }
    async fn suspend(&self, user_id: &model::CredentialId) -> Result<()> {
        let failed_logins = self.log(user_id).await?;
        if failed_logins.exceeded_limit() {
            let mut client = self.db.client().await?;
            let transaction = client.transaction().await?;
            transaction
                .execute(DELETE_FAILED_LOGIN_RECORD, &[&user_id])
                .await?;
            if !failed_logins.expired()? {
                transaction
                    .execute(credentials::query::SUSPEND, &[&user_id])
                    .await?;
            }
            transaction.commit().await?;
        }
        Ok(())
    }
}
