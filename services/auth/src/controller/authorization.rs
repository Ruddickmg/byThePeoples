use crate::{controller::password, model, repository, Error};

pub enum Results {
    Valid(model::Credentials),
    Suspended,
    Invalid,
    None,
}

pub async fn authorize<
    T: model::Database,
    L: repository::LoginHistory<T>,
    C: repository::Credentials<T>,
>(
    user_credentials: &model::NameRequest,
    auth_credentials: &C,
    login_history: &L,
) -> Result<Results, Error> {
    if let Some(auth_record) = auth_credentials.by_name(&user_credentials.name).await? {
        let user_id = &auth_record.id;
        if auth_record.suspended()? {
            Ok(Results::Suspended)
        } else {
            if password::authenticate(&user_credentials.password, &auth_record.hash)? {
                Ok(Results::Valid(auth_record))
            } else {
                login_history.suspend(user_id).await?;
                Ok(Results::Invalid)
            }
        }
    } else {
        Ok(Results::None)
    }
}

#[cfg(test)]
mod authorization_test {
    use crate::{
        model,
        utilities::test::mock::{MockCredentials, MockLoginHistory},
    };

    fn setup_state() -> model::ServiceState<
        model::DatabaseConnection,
        MockLoginHistory<model::DatabaseConnection>,
        MockCredentials<model::DatabaseConnection>,
    > {
        let mock_login_history = MockLoginHistory::<model::DatabaseConnection>::new();
        let mock_credentials = MockCredentials::<model::DatabaseConnection>::new();
        model::ServiceState::new(mock_login_history, mock_credentials)
    }

    #[actix_rt::test]
    async fn returns_suspended_if_the_auth_record_has_been_suspended() {
        // TODO
    }
}
