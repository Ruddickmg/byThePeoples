use crate::{model, model::credentials, model::Client, Error};
use async_trait::async_trait;
use std::marker::{Send, Sync};
use serde::export::PhantomData;

type CredentialResults = Result<Option<model::Credentials>, Error>;

struct Phantom;

#[derive(Clone)]
pub struct CredentialsRepository<'a: 'b, 'b, T=model::DatabaseConnection>
    where T: model::Database<'a, 'b>
{
    db: T,
    phantom: PhantomData<&'b &'a Phantom>
}

#[derive(Clone, Debug)]
pub enum Status {
    Deleted,
    Exists,
    None,
}

impl<'a: 'b, 'b, T> CredentialsRepository<'a, 'b, T> where T: model::Database<'a, 'b> {
    pub fn new(db: T) -> CredentialsRepository<'a, 'b, T> {
        CredentialsRepository { db, phantom: PhantomData }
    }
    pub async fn get_by_single_param(&'a self, query: &str, param: &str) -> CredentialResults {
        let client = self.db.client().await?;
        let statement = client.prepare(query).await?;
        let mut results = client
            .query::<model::Credentials>(&statement, &[&param])
            .await?;
        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(results.remove(0)))
        }
    }
    pub async fn by_name(&'a self, name: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::NAME, name)
            .await
    }
    pub async fn by_email(&'a self, email: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::EMAIL, email)
            .await
    }
    pub async fn get_status(&'a self, name: &str, email: &str) -> Result<Status, Error> {
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::DELETED_AT).await?;
        let stored_credentials = client
            .query::<credentials::DeletedAt>(&stmt, &[&name, &email])
            .await?;
        if stored_credentials.is_empty() {
            Ok(Status::None)
        } else {
            Ok(match stored_credentials.first() {
                Some(_) => Status::Deleted,
                None => Status::Exists,
            })
        }
    }
    pub async fn update_credentials(
        &'a self,
        credentials: &model::Credentials,
    ) -> Result<model::Credentials, Error> {
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
    pub async fn save_credentials(
        &'a self,
        credentials: &model::FullRequest,
    ) -> Result<model::Credentials, Error> {
        let model::FullRequest {
            name,
            email,
            password,
        } = credentials;
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::SAVE).await?;
        Ok(client
            .query::<model::Credentials>(&stmt, &[&name, &email, &password])
            .await?
            .remove(0))
    }
    pub async fn mark_as_deleted_by_email(&'a self, email: &str) -> Result<i32, Error> {
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::DELETE_BY_EMAIL).await?;
        Ok(client
            .query::<credentials::AffectedRows>(&stmt, &[&email])
            .await?
            .first()
            .map_or(0, |affected| affected.count))
    }
}

#[async_trait]
pub trait Credentials<'a>: Clone + Send + Sync {
    async fn by_name(&'a self, name: &str) -> CredentialResults;
    async fn by_email(&'a self, email: &str) -> CredentialResults;
    async fn get_status(&'a self, name: &str, email: &str) -> Result<Status, Error>;
    async fn update_credentials(
        &'a self,
        credentials: &model::Credentials,
    ) -> Result<model::Credentials, Error>;
    async fn save_credentials(
        &'a self,
        credentials: &model::FullRequest,
    ) -> Result<model::Credentials, Error>;
    async fn mark_as_deleted_by_email(&'a self, email: &str) -> Result<i32, Error>;
}

#[async_trait]
impl<'a: 'b, 'b, T: model::Database<'a, 'b>> Credentials<'a> for CredentialsRepository<'a, 'b, T> {
    async fn by_name(&'a self, name: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::NAME, name)
            .await
    }
    async fn by_email(&'a self, email: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::EMAIL, email)
            .await
    }
    async fn get_status(&'a self, name: &str, email: &str) -> Result<Status, Error> {
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::DELETED_AT).await?;
        let stored_credentials = client
            .query::<credentials::DeletedAt>(&stmt, &[&name, &email])
            .await?;
        if stored_credentials.is_empty() {
            Ok(Status::None)
        } else {
            Ok(match stored_credentials.first() {
                Some(_) => Status::Deleted,
                None => Status::Exists,
            })
        }
    }
    async fn update_credentials(
        &'a self,
        credentials: &model::Credentials,
    ) -> Result<model::Credentials, Error> {
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
    async fn save_credentials(
        &'a self,
        credentials: &model::FullRequest,
    ) -> Result<model::Credentials, Error> {
        let model::FullRequest {
            name,
            email,
            password,
        } = credentials;
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::SAVE).await?;
        Ok(client
            .query::<model::Credentials>(&stmt, &[&name, &email, &password])
            .await?
            .remove(0))
    }
    async fn mark_as_deleted_by_email(&'a self, email: &str) -> Result<i32, Error> {
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::DELETE_BY_EMAIL).await?;
        Ok(client
            .query::<credentials::AffectedRows>(&stmt, &[&email])
            .await?
            .first()
            .map_or(0, |affected| affected.count))
    }
}
