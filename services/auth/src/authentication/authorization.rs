use crate::{authentication::password, model, repository, Error};
use std::sync;

pub async fn authorize(
    user_credentials: model::AuthRequest,
    db: sync::MutexGuard<'_, model::Database>,
) -> Result<Option<model::Credentials>, Error> {
    let client = db.client().await?;
    let mut auth_credentials = repository::Credentials::new(client);
    if let Some(auth_record) = auth_credentials.by_name(&user_credentials.name).await? {
        if password::authenticate(&user_credentials.password, &auth_record.hash)? {
            return Ok(Some(auth_record));
        }
    }
    Ok(None)
}
