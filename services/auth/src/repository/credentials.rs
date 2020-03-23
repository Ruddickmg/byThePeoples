use crate::{model, Error};

type CredentialResults = Result<Option<model::Credentials>, Error>;

const GET_CREDENTIALS_BY_NAME: &str =
    "SELECT id, email, name, hash FROM auth.credentials WHERE name = $1";
const GET_CREDENTIALS_BY_EMAIL: &str =
    "SELECT id, email, name, hash FROM auth.credentials WHERE name = $1";
const SAVE_CREDENTIALS: &str =
    "INSERT INTO auth.credentials(name, email, hash) VALUES ($1, $2, $3)";
const CHECK_EXISTING_CREDENTIALS: &str =
    "SELECT id, email, name, hash, deleted_at FROM auth.credentials WHERE name = $1 OR email = $2";
const UPDATE_EXISTING_RECORD: &str =
    "UPDATE auth.credentials(name, password, updated_at, deleted_at) VALUES ($1, $2, CURRENT_TIMESTAMP, null) WHERE email = $3";

pub struct Credentials<'a> {
    client: database::Client<'a>,
}

pub enum Status {
    Deleted(model::Credentials),
    Exists,
    None,
}

impl<'a> Credentials<'a> {
    pub fn new(client: database::Client<'a>) -> Credentials {
        Credentials { client }
    }
    async fn get_by_single_param(&mut self, query: &str, param: &str) -> CredentialResults {
        let statement = self.client.prepare(query).await?;
        let rows = self.client.query(&statement, &[&param]).await?;
        Ok(model::Credentials::from(rows))
    }
    pub async fn by_name(&mut self, name: &str) -> CredentialResults {
        self.get_by_single_param(GET_CREDENTIALS_BY_NAME, name)
            .await
    }
    pub async fn by_email(&mut self, email: &str) -> CredentialResults {
        self.get_by_single_param(GET_CREDENTIALS_BY_EMAIL, email)
            .await
    }
    pub async fn credential_status(
        &mut self,
        credentials: &model::CredentialRequest,
    ) -> Result<Status, Error> {
        let model::CredentialRequest { name, email, .. } = credentials;
        let stmt = self.client.prepare(CHECK_EXISTING_CREDENTIALS).await?;
        let results = self.client.query(&stmt, &[&name, &email]).await?;
        Ok(match results.first() {
            Some(row) => match row.get::<usize, Option<String>>(5) {
                Some(_) => {
                    if let Some(credentials) = model::Credentials::from(results) {
                        if &credentials.email == email {
                            return Ok(Status::Deleted(credentials));
                        }
                        return Ok(Status::Exists);
                    };
                    return Err(Error::DatabaseError(database::Error::Error(String::from(
                        "Could not parse credentials from database while retrieving status",
                    ))));
                }
                None => Status::Exists,
            },
            None => Status::None,
        })
    }
    pub async fn update_credentials(
        &self,
        credentials: &model::Credentials,
    ) -> Result<database::Results, Error> {
        let model::Credentials {
            name, email, hash, ..
        } = credentials;
        let stmt = self.client.prepare(UPDATE_EXISTING_RECORD).await?;
        Ok(self.client.query(&stmt, &[&name, &hash, &email]).await?)
    }
    pub async fn save_credentials(
        &mut self,
        credentials: &model::CredentialRequest,
    ) -> Result<database::Results, Error> {
        let model::CredentialRequest {
            name,
            email,
            password,
        } = credentials;
        let stmt = self.client.prepare(SAVE_CREDENTIALS).await?;
        Ok(self
            .client
            .query(&stmt, &[&name, &email, &password])
            .await?)
    }
}
