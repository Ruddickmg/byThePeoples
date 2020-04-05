use crate::{model, model::credentials, Error};

type CredentialResults = Result<Option<model::Credentials>, Error>;

pub struct Credentials<'a> {
    client: database::Client<'a>,
}

pub enum Status {
    Deleted,
    Exists,
    None,
}

impl<'a> Credentials<'a> {
    pub fn new(client: database::Client<'a>) -> Credentials {
        Credentials { client }
    }
    async fn get_by_single_param(&mut self, query: &str, param: &str) -> CredentialResults {
        let statement = self.client.prepare(query).await?;
        let mut results = self
            .client
            .query::<model::Credentials>(&statement, &[&param])
            .await?;
        if results.is_empty() {
            Ok(None)
        } else {
            Ok(Some(results.remove(0)))
        }
    }
    pub async fn by_name(&mut self, name: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::NAME, name)
            .await
    }
    pub async fn by_email(&mut self, email: &str) -> CredentialResults {
        self.get_by_single_param(credentials::query::EMAIL, email)
            .await
    }
    pub async fn get_status(&mut self, credentials: &model::FullRequest) -> Result<Status, Error> {
        let model::FullRequest { name, email, .. } = credentials;
        let stmt = self.client.prepare(credentials::query::DELETED_AT).await?;
        let stored_credentials = self
            .client
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
        let stmt = self.client.prepare(credentials::query::UPDATE).await?;
        Ok(self
            .client
            .query::<model::Credentials>(&stmt, &[&name, &hash, &email, &id])
            .await?
            .remove(0))
    }
    pub async fn save_credentials(
        &mut self,
        credentials: &model::FullRequest,
    ) -> Result<Vec<credentials::AffectedRows>, Error> {
        let model::FullRequest {
            name,
            email,
            password,
        } = credentials;
        let stmt = self.client.prepare(credentials::query::SAVE).await?;
        Ok(self
            .client
            .query::<credentials::AffectedRows>(&stmt, &[&name, &email, &password])
            .await?)
    }
    pub async fn mark_as_deleted_by_email(&mut self, email: &str) -> Result<(), Error> {
        let stmt = self
            .client
            .prepare(credentials::query::DELETE_BY_EMAIL)
            .await?;
        self.client
            .query::<credentials::AffectedRows>(&stmt, &[&email])
            .await?;
        Ok(())
    }
}
