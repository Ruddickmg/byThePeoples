use crate::{controller::password, model, repository, Error};

pub enum SaveResults {
    WeakPassword(password::PasswordIssues),
    Success(model::Credentials),
    Conflict,
}

pub async fn create(
    db: &model::Database,
    model::FullRequest {
        name,
        email,
        password,
    }: model::FullRequest,
) -> Result<SaveResults, Error> {
    let password_strength = password::strength(&name, &email, &password)?;
    if let password::Strength::Weak(problems) = password_strength {
        Ok(SaveResults::WeakPassword(problems))
    } else {
        let client = db.client().await?;
        let mut credentials = repository::Credentials::new(&client);
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
                Some(stored_credentials) => Ok(SaveResults::Success(stored_credentials)),
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
    Suspended,
    Unauthorized,
}

pub async fn delete(
    db: &model::Database,
    model::EmailRequest { password, email }: model::EmailRequest,
) -> Result<DeleteResults, Error> {
    let client = db.client().await?;
    let mut credentials = repository::Credentials::new(&client);
    let failed_login = repository::FailedLogin::new(&client);
    if let Some(stored_credentials) = credentials.by_email(&email).await? {
        if stored_credentials.suspended()? {
            Ok(DeleteResults::Suspended)
        } else {
            if password::authenticate(&password, &stored_credentials.hash)? {
                credentials.mark_as_deleted_by_email(&email).await?;
                Ok(DeleteResults::Success)
            } else {
                failed_login.suspend(&stored_credentials.id).await?;
                Ok(DeleteResults::Unauthorized)
            }
        }
    } else {
        Ok(DeleteResults::NotFound)
    }
}

pub enum UpdateResults {
    Success(model::Credentials),
    NotFound,
    Suspended,
    Unauthorized,
}

pub async fn update(
    db: &model::Database,
    auth_details: &model::EmailRequest,
    model::CredentialsRequest {
        name,
        email,
        password,
    }: &model::CredentialsRequest,
) -> Result<UpdateResults, Error> {
    let client = db.client().await?;
    let mut credentials = repository::Credentials::new(&client);
    let failed_login = repository::FailedLogin::new(&client);
    if let Some(stored_credentials) = credentials.by_email(&auth_details.email).await? {
        if stored_credentials.suspended()? {
            Ok(UpdateResults::Suspended)
        } else {
            if password::authenticate(&auth_details.password, &stored_credentials.hash)? {
                let updated_credentials = credentials
                    .update_credentials(&model::Credentials {
                        name: name.as_ref().map_or(stored_credentials.name, String::from),
                        email: email
                            .as_ref()
                            .map_or(stored_credentials.email, String::from),
                        hash: match &password {
                            Some(p) => password::hash_password(p)?,
                            None => stored_credentials.hash,
                        },
                        ..stored_credentials
                    })
                    .await?;
                Ok(UpdateResults::Success(updated_credentials))
            } else {
                failed_login.suspend(&stored_credentials.id).await?;
                Ok(UpdateResults::Unauthorized)
            }
        }
    } else {
        Ok(UpdateResults::NotFound)
    }
}
