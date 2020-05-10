use crate::{model, model::credentials, Error};

type CredentialResults = Result<Option<model::Credentials>, Error>;

#[derive(Clone)]
pub struct Credentials<T: model::Database> {
    db: T,
}

pub enum Status {
    Deleted,
    Exists,
    None,
}

impl<T: model::Database> Credentials<T> {
    pub fn new(db: T) -> Credentials<T> {
        Credentials { db }
    }
    async fn get_by_single_param(&self, query: &str, param: &str) -> CredentialResults {
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
    pub async fn by_name(&self, name: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::NAME, name)
            .await
    }
    pub async fn by_email(&self, email: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::EMAIL, email)
            .await
    }
    pub async fn get_status(&self, credentials: &model::FullRequest) -> Result<Status, Error> {
        let client = self.db.client().await?;
        let model::FullRequest { name, email, .. } = credentials;
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
        &self,
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
    pub async fn save_credentials(&self, credentials: &model::FullRequest) -> Result<i32, Error> {
        let model::FullRequest {
            name,
            email,
            password,
        } = credentials;
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::SAVE).await?;
        Ok(client
            .query::<credentials::AffectedRows>(&stmt, &[&name, &email, &password])
            .await?
            .first()
            .map_or(0, |affected| affected.count))
    }
    pub async fn mark_as_deleted_by_email(&self, email: &str) -> Result<i32, Error> {
        let client = self.db.client().await?;
        let stmt = client.prepare(credentials::query::DELETE_BY_EMAIL).await?;
        Ok(client
            .query::<credentials::AffectedRows>(&stmt, &[&email])
            .await?
            .first()
            .map_or(0, |affected| affected.count))
    }
}
