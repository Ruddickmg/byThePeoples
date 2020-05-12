extern crate btp_auth_server;

use actix_web::web;
use btp_auth_server::{
    configuration::database::TEST_DATABASE_CONFIG,
    model,
    model::{credentials::query::SUSPEND, CredentialId},
    Error,
};
use fake::faker::{internet::en as internet, name::en as name};
use fake::Fake;

const DATABASE_INITIALIZATION_FAILURE: &str = "Failed to initialize database";
const APP_STATE_CREATION_FAILURE: &str = "Failed to create application state";
const APP_STATE_INITIALIZATION_FAILURE: &str = "Failed to initialize application state";
const CREATE_OR_UPDATE_FAILED_LOGIN: &str = "INSERT INTO auth.failed_login(user_id, attempts, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP);";
const GET_FAILED_LOGIN_HISTORY: &str =
    "SELECT user_id, attempts, created_at, updated_at FROM auth.failed_login WHERE user_id = $1;";
const CREATE_FAILED_LOGIN: &str = "INSERT INTO auth.failed_login(user_id, created_at, updated_at, attempts) VALUES ($1, $2, $3, $4);";
const MAX_FAKE_PASSWORD_LENGTH: usize = 20;
const MIN_FAKE_PASSWORD_LENGTH: usize = 15;

pub async fn init_data() -> web::Data<model::ServiceState<model::DatabaseConnection>> {
    let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
        .await
        .expect(DATABASE_INITIALIZATION_FAILURE);
    let state = model::ServiceState::new(db)
        .await
        .expect(APP_STATE_CREATION_FAILURE)
        .initialize()
        .await
        .expect(APP_STATE_INITIALIZATION_FAILURE);
    web::Data::new(state)
}

pub fn fake_credentials() -> (String, String, String) {
    let name = name::Name().fake();
    let email = name::FirstName().fake();
    let password = internet::Password(MIN_FAKE_PASSWORD_LENGTH..MAX_FAKE_PASSWORD_LENGTH).fake();
    (name, email, password)
}

pub struct Helper {
    db: model::DatabaseConnection,
    state: model::ServiceState<model::DatabaseConnection>,
}

impl Helper {
    pub async fn new() -> Result<Helper, Error> {
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG).await?;
        Ok(Helper {
            db: db.clone(),
            state: model::ServiceState::new(db.clone()).await?,
        })
    }
    pub async fn get_credentials_by_name(
        &self,
        name: &str,
    ) -> Result<Option<model::Credentials>, Error> {
        Ok(self.state.credentials.by_name(&name).await?)
    }
    pub async fn add_credentials(&self, request: &model::FullRequest) {
        let model::FullRequest {
            name,
            email,
            password,
        }: &model::FullRequest = request;
        let query =
            String::from("INSERT INTO auth.credentials(name, hash, email) VALUES ($1, $2, $3)");
        let db = &self.db;
        db.client()
            .await
            .unwrap()
            .execute(&query, &[&name, &password, &email])
            .await
            .unwrap();
    }
    pub async fn delete_credentials_by_name(&self, name: &str) {
        let db = &self.db;
        db.client()
            .await
            .unwrap()
            .execute("DELETE FROM auth.credentials WHERE name = $1", &[&name])
            .await
            .unwrap();
    }
    pub async fn suspend_user(&self, user_id: &CredentialId) {
        let db = &self.db;
        db.client()
            .await
            .unwrap()
            .execute(SUSPEND, &[&user_id])
            .await
            .unwrap();
    }
    pub async fn set_login_attempts(&self, user_id: &CredentialId, attempts: &database::SmallInt) {
        let db = &self.db;
        db.client()
            .await
            .unwrap()
            .execute(CREATE_OR_UPDATE_FAILED_LOGIN, &[&user_id, &attempts])
            .await
            .unwrap();
    }
    pub async fn set_login_history(&self, failed_login_record: &model::FailedLogin) {
        let db = &self.db;
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
        let db = &self.db;
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