use crate::{controller::password, model, repository, Error};

pub enum SaveResults {
    WeakPassword,
    Conflict,
    Saved(model::Credentials),
}

pub async fn create(
    db: &model::Database,
    model::FullRequest {
        name,
        email,
        password,
    }: model::FullRequest,
) -> Result<SaveResults, Error> {
    if password::Strength::Weak >= password::strength(&password) {
        Ok(SaveResults::WeakPassword)
    } else {
        let client = db.client().await?;
        let mut credentials = repository::Credentials::new(client);
        let hash = password::hash_password(&password)?;
        let encrypted_credentials = model::FullRequest {
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
    model::EmailRequest { password, email }: model::EmailRequest,
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

pub enum UpdateResults {
    Success(model::Credentials),
    NotFound,
    Unauthorized,
}

pub async fn update(
    db: &model::Database,
    auth_details: &model::EmailRequest,
    credential_updates: &model::CredentialsRequest,
) -> Result<UpdateResults, Error> {
    let client = db.client().await?;
    let mut credentials = repository::Credentials::new(client);
    if let Some(stored_credentials) = credentials.by_email(&auth_details.email).await? {
        if password::authenticate(&auth_details.password, &stored_credentials.hash)? {
            let updated_credentials = credentials
                .update_credentials(&model::Credentials {
                    name: match &credential_updates.name {
                        Some(name) => String::from(name),
                        None => String::from(&stored_credentials.name),
                    },
                    email: match &credential_updates.email {
                        Some(email) => String::from(email),
                        None => String::from(&stored_credentials.name),
                    },
                    hash: match &credential_updates.password {
                        Some(updated_password) => {
                            String::from(password::hash_password(updated_password)?)
                        }
                        None => String::from(&stored_credentials.hash),
                    },
                    ..stored_credentials
                })
                .await?;
            Ok(UpdateResults::Success(updated_credentials))
        } else {
            Ok(UpdateResults::Unauthorized)
        }
    } else {
        Ok(UpdateResults::NotFound)
    }
}
