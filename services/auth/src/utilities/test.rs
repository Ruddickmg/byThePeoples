use crate::{model, repository, Error};
use actix_web::{test, web, FromRequest};
use fake::faker::{internet::en as internet, name::en as name};
use fake::Fake;
use serde::{Deserialize, Serialize};

const MAX_FAKE_PASSWORD_LENGTH: usize = 20;
const MIN_FAKE_PASSWORD_LENGTH: usize = 15;

pub struct Helper {
    state: model::ServiceState,
}

pub fn fake_credentials() -> (String, String, String) {
    let name = name::Name().fake();
    let email = name::FirstName().fake();
    let password = internet::Password(MIN_FAKE_PASSWORD_LENGTH..MAX_FAKE_PASSWORD_LENGTH).fake();
    (name, email, password)
}

impl Helper {
    pub async fn new() -> Result<Helper, Error> {
        Ok(Helper {
            state: model::ServiceState::new().await?,
        })
    }
    pub async fn get_credentials_by_name(
        &self,
        name: &str,
    ) -> Result<Option<model::Credentials>, Error> {
        let db = &self.state.db;
        let client = db.client().await?;
        let mut credentials = repository::Credentials::new(&client);
        Ok(credentials.by_name(&name).await?)
    }
    pub async fn add_credentials(
        &self,
        model::FullRequest {
            name,
            email,
            password,
        }: &model::FullRequest,
    ) {
        let query =
            String::from("INSERT INTO auth.credentials(name, hash, email) VALUES ($1, $2, $3)");
        let db = &self.state.db;
        db.client()
            .await
            .unwrap()
            .execute(&query, &[&name, &password, &email])
            .await
            .unwrap();
    }
    pub async fn delete_credentials_by_name(&self, name: &str) {
        let db = &self.state.db;
        db.client()
            .await
            .unwrap()
            .execute("DELETE FROM auth.credentials WHERE name = $1", &[&name])
            .await
            .unwrap();
    }
}
