use crate::{controller::password, model, repository, Error};

pub enum Results {
    Valid(model::Credentials),
    Suspended,
    Invalid,
    None,
}

pub async fn authorize<T: model::Database>(
    user_credentials: &model::NameRequest,
    db: &T,
) -> Result<Results, Error> {
    let client = db.client().await?;
    let mut auth_credentials = repository::Credentials::new(&client);
    let failed_login = repository::FailedLogin::new(&client);
    if let Some(auth_record) = auth_credentials.by_name(&user_credentials.name).await? {
        let user_id = &auth_record.id;
        if auth_record.suspended()? {
            Ok(Results::Suspended)
        } else {
            if password::authenticate(&user_credentials.password, &auth_record.hash)? {
                Ok(Results::Valid(auth_record))
            } else {
                failed_login.suspend(user_id).await?;
                Ok(Results::Invalid)
            }
        }
    } else {
        Ok(Results::None)
    }
}
