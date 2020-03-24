use crate::{model, repository, Error};
use fake::faker::name::en as name;
use fake::Fake;

pub struct Helper {
    state: model::ServiceState,
}

impl Helper {
    pub async fn new() -> Result<Helper, Error> {
        Ok(Helper {
            state: model::ServiceState::new().await?,
        })
    }
    pub fn fake_credentials(&self) -> (String, String, String) {
        let name = name::Name().fake();
        let email = name::FirstName().fake();
        let password = name::LastName().fake();
        (name, email, password)
    }
    pub async fn get_credentials_by_name(
        &self,
        name: &str,
    ) -> Result<Option<model::Credentials>, Error> {
        let db = self.state.db.lock().unwrap();
        let client = db.client().await?;
        let mut credentials = repository::Credentials::new(client);
        Ok(credentials.by_name(&name).await?)
    }
    pub async fn add_credentials(
        &self,
        model::CredentialRequest {
            name,
            email,
            password,
        }: model::CredentialRequest,
    ) {
        let query =
            String::from("INSERT INTO auth.credentials(name, hash, email) VALUES ($1, $2, $3)");
        let db = self.state.db.lock().unwrap();
        db.client()
            .await
            .unwrap()
            .execute(&query, &[&name, &password, &email])
            .await
            .unwrap();
    }
    pub async fn delete_credentials_by_name(&self, name: &str) {
        let db = self.state.db.lock().unwrap();
        db.client()
            .await
            .unwrap()
            .execute("DELETE FROM auth.credentials WHERE name = $1", &[&name])
            .await
            .unwrap();
    }
}
