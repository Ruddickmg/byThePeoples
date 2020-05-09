use crate::{
    model,
    model::{credentials::query::SUSPEND, CredentialId},
    repository, Error,
};
use fake::faker::{internet::en as internet, name::en as name};
use fake::Fake;

const CREATE_OR_UPDATE_FAILED_LOGIN: &str = "INSERT INTO auth.failed_login(user_id, attempts, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP);";
const GET_FAILED_LOGIN_HISTORY: &str =
    "SELECT user_id, attempts, created_at, updated_at FROM auth.failed_login WHERE user_id = $1;";
const CREATE_FAILED_LOGIN: &str = "INSERT INTO auth.failed_login(user_id, created_at, updated_at, attempts) VALUES ($1, $2, $3, $4);";
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
    pub async fn suspend_user(&self, user_id: &CredentialId) {
        let db = &self.state.db;
        db.client()
            .await
            .unwrap()
            .execute(SUSPEND, &[&user_id])
            .await
            .unwrap();
    }
    pub async fn set_login_attempts(&self, user_id: &CredentialId, attempts: &database::SmallInt) {
        let db = &self.state.db;
        db.client()
            .await
            .unwrap()
            .execute(CREATE_OR_UPDATE_FAILED_LOGIN, &[&user_id, &attempts])
            .await
            .unwrap();
    }
    pub async fn set_login_history(&self, failed_login_record: &model::FailedLogin) {
        let db = &self.state.db;
        let model::FailedLogin {
            user_id,
            updated_at,
            created_at,
            attempts,
        } = failed_login_record;
        db.client()
            .await
            .unwrap()
            .execute(
                CREATE_FAILED_LOGIN,
                &[&user_id, &updated_at, &created_at, &attempts],
            )
            .await
            .unwrap();
    }
    pub async fn get_login_history(
        &self,
        user_id: &CredentialId,
    ) -> Result<Vec<model::FailedLogin>, Error> {
        let db = &self.state.db;
        let client = &db.client().await?;
        let stmt = client.prepare(GET_FAILED_LOGIN_HISTORY).await?;
        Ok(client
            .query(&stmt, &[&user_id])
            .await?
            .iter()
            .map(model::FailedLogin::from)
            .collect())
    }
}
