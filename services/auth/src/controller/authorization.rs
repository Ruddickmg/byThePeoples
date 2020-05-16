use crate::{controller::password, model, repository, Error};

#[derive(Eq, PartialEq, Debug)]
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
    use super::*;
    use crate::{model, utilities::test::mock::service_state};
    use std::time::SystemTime;

    fn fake_credentials() -> model::Credentials {
        model::Credentials {
            id: 1,
            email: "email".to_string(),
            name: "joey".to_string(),
            hash: "hash".to_string(),
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            deleted_at: None,
            locked_at: None,
        }
    }

    #[actix_rt::test]
    async fn returns_suspended_if_the_auth_record_has_been_suspended() {
        let mut state = service_state();
        let request = model::NameRequest {
            password: "king kong".to_string(),
            name: "dude man".to_string(),
        };
        let mut credentials = fake_credentials();
        credentials.locked_at = Some(SystemTime::now());
        state.credentials.by_name.returns(Some(credentials));
        let result = authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(result, Results::Suspended);
    }

    #[actix_rt::test]
    async fn returns_none_if_no_record_is_found() {
        let mut state = service_state();
        let request = model::NameRequest {
            password: "king kong".to_string(),
            name: "dude man".to_string(),
        };
        state.credentials.by_name.returns(None);
        let result = authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(result, Results::None);
    }

    #[actix_rt::test]
    async fn returns_invalid_if_credentials_dont_match() {
        let mut state = service_state();
        let request = model::NameRequest {
            password: "king kong".to_string(),
            name: "dude man".to_string(),
        };
        let record = fake_credentials();
        state.login_history.suspend.returns(());
        state.credentials.by_name.returns(Some(record));
        let result = authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(result, Results::Invalid);
    }

    #[actix_rt::test]
    async fn calls_suspend_on_a_user_if_their_credentials_are_invalid() {
        let mut state = service_state();
        let request = model::NameRequest {
            password: "king kong".to_string(),
            name: "dude man".to_string(),
        };
        let record = fake_credentials();
        state.login_history.suspend.returns(());
        state.credentials.by_name.returns(Some(record));
        authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(state.login_history.suspend.times_called(), 1);
    }
}
