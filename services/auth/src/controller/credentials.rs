use crate::{controller::password, model, repository, Error};

pub enum SaveResults {
    WeakPassword,
    Conflict,
    Saved(model::Credentials),
}

pub async fn save(
    db: &model::Database,
    model::CredentialRequest {
        name,
        email,
        password,
    }: model::CredentialRequest,
) -> Result<SaveResults, Error> {
    if password::Strength::Weak >= password::strength(&password) {
        Ok(SaveResults::WeakPassword)
    } else {
        let client = db.client().await?;
        let mut credentials = repository::Credentials::new(client);
        let hash = password::hash_password(&password)?;
        let encrypted_credentials = model::CredentialRequest {
            name: String::from(&name),
            email: String::from(&email),
            password: String::from(&hash),
        };
        if let repository::credentials::Status::None =
            credentials.get_status(&encrypted_credentials).await?
        {
            credentials.save_credentials(&encrypted_credentials).await?;
            match credentials.by_name(&name).await? {
                Some(stored_credentials) => Ok(SaveResults::Saved(stored_credentials)),
                None => Err(Error::DatabaseError(database::Error::Error(String::from(
                    "Could not retrieve credentials after save",
                )))),
            }
        } else {
            Ok(SaveResults::Conflict)
        }
    }
}

pub enum DeleteResults {
    Success,
    NotFound,
    Unauthorized,
}

pub async fn delete(
    db: &model::Database,
    model::CredentialRequest {
        password, email, ..
    }: model::CredentialRequest,
) -> Result<DeleteResults, Error> {
    let client = db.client().await?;
    let mut credentials = repository::Credentials::new(client);
    if let Some(stored_credentials) = credentials.by_email(&email).await? {
        if password::authenticate(&password, &stored_credentials.hash)? {
            credentials.mark_as_deleted_by_email(&email).await?;
            Ok(DeleteResults::Success)
        } else {
            Ok(DeleteResults::Unauthorized)
        }
    } else {
        Ok(DeleteResults::NotFound)
    }
}
