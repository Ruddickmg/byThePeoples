use crate::{
    authentication::{jwt, password},
    model, repository, Error,
};
use actix_web::{web, HttpResponse};
use std::sync::MutexGuard;

async fn handle_credential_saving(
    db: MutexGuard<'_, model::Database>,
    model::CredentialRequest {
        name,
        email,
        password,
    }: model::CredentialRequest,
) -> Result<Option<model::Credentials>, Error> {
    let client = db.client().await?;
    let mut credentials = repository::Credentials::new(client);
    let hash = password::hash_password(&password)?;
    let encrypted_credentials = model::CredentialRequest {
        name: String::from(&name),
        email: String::from(&email),
        password: String::from(&hash),
    };
    match credentials
        .credential_status(&encrypted_credentials)
        .await?
    {
        repository::credentials::Status::Exists => Ok(None),
        repository::credentials::Status::Deleted(deleted_credentials) => {
            let updated_credentials = model::Credentials {
                email: String::from(&email),
                name: String::from(&name),
                hash: String::from(&hash),
                id: deleted_credentials.id,
            };
            credentials.update_credentials(&updated_credentials).await?;
            Ok(Some(updated_credentials))
        }
        repository::credentials::Status::None => {
            credentials.save_credentials(&encrypted_credentials).await?;
            Ok(credentials.by_name(&name).await?)
        }
    }
}

pub async fn save_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::CredentialRequest>,
) -> HttpResponse {
    let credential = model::CredentialRequest::from(json);
    let db = state.db.lock().unwrap();
    if let Ok(result) = handle_credential_saving(db, credential).await {
        match result {
            Some(stored_credentials) => {
                if let Ok(response) =
                    jwt::set_auth_header(HttpResponse::Created(), stored_credentials)
                {
                    return response;
                }
            }
            None => return HttpResponse::Conflict().finish(),
        }
    }
    HttpResponse::InternalServerError().finish()
}
