use crate::{model, Error};

const GET_FAILED_LOGIN: &str =
    "SELECT user_id, attempts, created_at, updated_at FROM auth.failed_login WHERE user_id = $1";
const CREATE_OR_UPDATE_FAILED_LOGIN: &str = "INSERT INTO auth.failed_login(user_id) VALUES ($1)
 ON CONFLICT DO
     UPDATE
     SET
         attempts = attempts + 1,
         updated_at = CURRENT_TIMESTAMP
RETURNING user_id, attempts, created_at, updated_at;";
const DELETE_FAILED_LOGIN_RECORD: &str = "DELETE FROM auth.failed_login WHERE user_id = $1";

pub struct FailedLogin<'a> {
    client: &'a database::Client<'a>,
}

impl<'a> FailedLogin<'a> {
    pub fn new(client: &'a database::Client<'a>) -> FailedLogin {
        FailedLogin { client }
    }
    pub async fn log(&self, id: &model::CredentialId) -> Result<model::FailedLogin, Error> {
        let stmt = self.client.prepare(CREATE_OR_UPDATE_FAILED_LOGIN).await?;
        Ok(self
            .client
            .query::<model::FailedLogin>(&stmt, &[&id])
            .await?
            .remove(0))
    }
    pub async fn get(&self, id: &model::CredentialId) -> Result<model::FailedLogin, Error> {
        let stmt = self.client.prepare(GET_FAILED_LOGIN).await?;
        Ok(self
            .client
            .query::<model::FailedLogin>(&stmt, &[&id])
            .await?
            .remove(0))
    }
    pub async fn delete(&self, id: &model::CredentialId) -> Result<(), Error> {
        self.client
            .execute(DELETE_FAILED_LOGIN_RECORD, &[&id])
            .await?;
        Ok(())
    }
}
