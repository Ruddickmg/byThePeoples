use crate::{controller::password, model, repository, Error};
use futures::future::join;

pub enum Results {
    Valid(model::Credentials),
    Suspended,
    Invalid,
    None,
}

pub async fn authorize(
    user_credentials: &model::NameRequest,
    db: &model::Database,
) -> Result<Results, Error> {
    let client = db.client().await?;
    let mut auth_credentials = repository::Credentials::new(&client);
    let mut failed_login = repository::FailedLogin::new(&client);
    if let Some(auth_record) = auth_credentials.by_name(&user_credentials.name).await? {
        if auth_record.suspended()? {
            Ok(Results::Suspended)
        } else {
            if password::authenticate(&user_credentials.password, &auth_record.hash)? {
                Ok(Results::Valid(auth_record))
            } else {
                let user_id = &auth_record.id;
                let failed_logins = failed_login.log(user_id).await?;
                if failed_logins.exceeded_limit() {
                    join(
                        auth_credentials.suspend(user_id),
                        failed_login.delete(user_id),
                    )
                    .await;
                };
                Ok(Results::Invalid)
            }
        }
    } else {
        Ok(Results::None)
    }
}
