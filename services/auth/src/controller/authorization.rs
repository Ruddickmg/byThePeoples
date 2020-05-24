use crate::{controller::password, model, repository, Error};

#[derive(Eq, PartialEq, Debug)]
pub enum Results {
    Valid(model::Credentials),
    Suspended,
    Invalid,
    None,
}

pub async fn authorize<
    L: repository::LoginHistory,
    C: repository::Credentials,
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
    use crate::utilities::test::fake;
    use std::time::SystemTime;

    #[actix_rt::test]
    async fn returns_suspended_if_the_auth_record_has_been_suspended() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        let mut credentials = fake::credentials();
        credentials.locked_at = Some(SystemTime::now());
        state.credentials.by_name.returns(Some(credentials));
        let result = authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(result, Results::Suspended);
    }

    #[actix_rt::test]
    async fn returns_none_if_no_record_is_found() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        state.credentials.by_name.returns(None);
        let result = authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(result, Results::None);
    }

    #[actix_rt::test]
    async fn returns_invalid_if_credentials_dont_match() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        let record = fake::credentials();
        state.login_history.suspend.returns(());
        state.credentials.by_name.returns(Some(record));
        let result = authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(result, Results::Invalid);
    }

    #[actix_rt::test]
    async fn calls_suspend_on_a_user_if_their_credentials_are_invalid() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        let record = fake::credentials();
        state.login_history.suspend.returns(());
        state.credentials.by_name.returns(Some(record));
        authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(state.login_history.suspend.times_called(), 1);
    }

    #[actix_rt::test]
    async fn returns_valid_if_credentials_match() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        let mut record = fake::credentials();
        record.hash = password::hash_password(&request.password).unwrap();
        state.credentials.by_name.returns(Some(record.clone()));
        let result = authorize(&request, &state.credentials, &state.login_history)
            .await
            .expect("error occurred in authorize");
        assert_eq!(result, Results::Valid(record.clone()));
    }
}
