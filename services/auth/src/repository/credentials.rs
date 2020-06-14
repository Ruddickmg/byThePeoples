use crate::{model, model::credentials, Result};
use async_trait::async_trait;
use std::marker::{Send, Sync};

type CredentialResults = Result<Option<model::Credentials>>;
pub type AppCredentials = CredentialsRepository<model::DatabaseConnection>;

#[derive(Clone)]
pub struct CredentialsRepository<T: model::Database> {
    db: T,
}

#[derive(Clone, Debug)]
pub enum CredentialStatus {
    Deleted,
    Exists,
    None,
}

impl<T: model::Database> CredentialsRepository<T> {
    pub fn new(db: T) -> CredentialsRepository<T> {
        CredentialsRepository { db }
    }
    async fn get_by_single_param(&self, query: &str, param: &str) -> CredentialResults {
        let client = self.db.client().await?;
        let statement = client.prepare(query).await?;
        Ok(client
            .query::<model::Credentials>(&statement, &[&param])
            .await?
            .first()
            .map(| credentials | credentials.clone()))
    }
    pub async fn by_name(&self, name: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::NAME, name)
            .await
    }
}

#[async_trait]
pub trait Credentials: Clone + Send + Sync {
    async fn by_name(&self, name: &str) -> CredentialResults;
    async fn by_email(&self, email: &str) -> CredentialResults;
    async fn by_id(&self, id: i32) -> CredentialResults;
    async fn get_status(&self, name: &str, email: &str) -> Result<CredentialStatus>;
    async fn update_credentials(
        &self,
        credentials: &model::Credentials,
    ) -> Result<model::Credentials>;
    async fn update_password_hash(&self, id: &i32, hash: &str) -> Result<model::Credentials>;
    async fn save_credentials(
        &self,
        credentials: &model::FullRequest,
    ) -> Result<model::Credentials>;
    async fn mark_as_deleted_by_email(&self, email: &str) -> Result<i32>;
}

#[async_trait]
impl<T: model::Database> Credentials for CredentialsRepository<T> {
    async fn by_name(&self, name: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::NAME, name)
            .await
    }
    async fn by_email(&self, email: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::EMAIL, email)
            .await
    }
    async fn by_id(&self, id: i32) -> CredentialResults {
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::ID).await?;
        let mut results = client
            .query::<model::Credentials>(&stmt, &[&id])
            .await?;
        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(results.remove(0)))
        }
    }
    async fn get_status(&self, name: &str, email: &str) -> Result<CredentialStatus> {
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::DELETED_AT).await?;
        let stored_credentials = client
            .query::<credentials::DeletedAt>(&stmt, &[&name, &email])
            .await?;
        if stored_credentials.is_empty() {
            Ok(CredentialStatus::None)
        } else {
            Ok(match stored_credentials.first() {
                Some(_) => CredentialStatus::Deleted,
                None => CredentialStatus::Exists,
            })
        }
    }
    async fn update_credentials(
        &self,
        credentials: &model::Credentials,
    ) -> Result<model::Credentials> {
        let model::Credentials {
            name,
            email,
            hash,
            id,
            ..
        } = credentials;
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::UPDATE).await?;
        Ok(client
            .query::<model::Credentials>(&stmt, &[&name, &hash, &email, &id])
            .await?
            .remove(0))
    }
    async fn update_password_hash(&self, id: &i32, hash: &str) -> Result<model::Credentials> {
        let client = &self.db.client().await?;
        let stmt = client.prepare(credentials::query::UPDATE_PASSWORD_HASH)
            .await?;
        Ok(client.query::<model::Credentials>(&stmt, &[&id, &hash]).await?.remove(0))
    }
    async fn save_credentials(
        &self,
        credentials: &model::FullRequest,
    ) -> Result<model::Credentials> {
        let model::FullRequest {
            name,
            email,
            password,
        } = credentials;
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::CREATE).await?;
        Ok(client
            .query::<model::Credentials>(&stmt, &[&name, &email, &password])
            .await?
            .remove(0))
    }
    async fn mark_as_deleted_by_email(&self, email: &str) -> Result<i32> {
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::DELETE_BY_EMAIL).await?;
        Ok(client
            .query::<credentials::AffectedRows>(&stmt, &[&email])
            .await?
            .first()
            .map_or(0, |affected| affected.count))
    }
}
