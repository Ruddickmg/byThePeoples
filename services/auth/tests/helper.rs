extern crate btp_auth_server;

use actix_web::web;
use btp_auth_server::{
    configuration::database::TEST_DATABASE_CONFIG,
    model::{
        credentials::query::SUSPEND,
        CredentialId,
    },
    Result,
    model,
};
use fake::faker::{internet::en as internet, name::en as name};
use fake::Fake;

const DATABASE_INITIALIZATION_FAILURE: &str = "Failed to initialize database";
const CREATE_OR_UPDATE_FAILED_LOGIN: &str = "INSERT INTO auth.failed_login(user_id, attempts, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP);";
const GET_FAILED_LOGIN_HISTORY: &str =
    "SELECT user_id, attempts, created_at, updated_at FROM auth.failed_login WHERE user_id = $1;";
const CREATE_FAILED_LOGIN: &str = "INSERT INTO auth.failed_login(user_id, created_at, updated_at, attempts) VALUES ($1, $2, $3, $4);";
const GET_RESET_REQUEST_BY_USER_ID: &str = "SELECT id, user_id, reset_token, name, email, created_at FROM auth.password_reset WHERE user_id = $1";
const CREATE_RESET_REQUEST: &str = "INSERT INTO auth.password_reset(id, user_id, reset_token, name, email, created_at) VALUES($1, $2, $3, $4, $5, $6) RETURNING id, user_id, reset_token, name, email, created_at";

const MAX_FAKE_PASSWORD_LENGTH: usize = 20;
const MIN_FAKE_PASSWORD_LENGTH: usize = 15;

pub const WEAK_PASSWORD: &str = "password";

pub async fn init_data() -> web::Data<model::AppServiceState> {
    let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
        .await
        .expect(DATABASE_INITIALIZATION_FAILURE);
    web::Data::new(model::initialize_state(&db))
}

pub fn fake_credentials() -> (String, String, String) {
    let name = name::Name().fake();
    let email = internet::FreeEmail().fake();
    let password = internet::Password(MIN_FAKE_PASSWORD_LENGTH..MAX_FAKE_PASSWORD_LENGTH).fake();
    (name, email, password)
}

pub struct Helper {
    db: model::DatabaseConnection,
    state: model::AppServiceState,
}

impl Helper {
    pub async fn new() -> Result<Helper> {
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG).await?;
        Ok(Helper {
            db: db.clone(),
            state: model::initialize_state(&db),
        })
    }
    pub async fn get_credentials_by_name(
        &self,
        name: &str,
    ) -> Result<Option<model::Credentials>> {
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
    ) -> Result<Vec<model::FailedLogin>> {
        let db = &self.db;
        let client = &db.client().await?;
        let stmt = client.prepare(GET_FAILED_LOGIN_HISTORY).await?;
        Ok(client.query::<model::FailedLogin>(&stmt, &[&user_id]).await?)
    }
    pub async fn get_reset_request(
        &self,
        user_id: &CredentialId,
    ) -> Result<model::PasswordResetRequest> {
        let db = &self.db;
        let client = &db.client().await?;
        let stmt = client.prepare(GET_RESET_REQUEST_BY_USER_ID).await?;
        Ok(client.query::<model::PasswordResetRequest>(&stmt, &[&user_id]).await?.remove(0))
    }
    pub async fn add_reset_request(
        &self,
        request: &model::PasswordResetRequest,
    ) -> Result<model::PasswordResetRequest> {
        let db = &self.db;
        let client = &db.client().await?;
        let stmt = client.prepare(CREATE_RESET_REQUEST).await?;
        Ok(client.query::<model::PasswordResetRequest>(
            &stmt,
            &[
                &request.id,
                &request.user_id,
                &request.reset_token,
                &request.name,
                &request.email,
                &request.created_at,
            ],
        ).await?
            .first()
            .unwrap()
            .clone())
    }
}
