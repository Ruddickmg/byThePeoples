use crate::{
    authentication::{jwt, password},
    model::{credentials, AuthRequest, Database},
    Error,
};
use std::sync;

pub async fn authorize(
    user_credentials: AuthRequest,
    db: sync::MutexGuard<'_, Database>,
) -> Result<Option<String>, Error> {
    let client = db.client().await?;
    let mut auth_credentials = credentials::Model::new(client);
    if let Some(auth_record) = auth_credentials.by_name(&user_credentials.name).await? {
        if password::authenticate(&user_credentials.password, &auth_record.hash)? {
            return Ok(Some(String::from(jwt::generate_token(auth_record)?)));
        }
    }
    Ok(None)
}
