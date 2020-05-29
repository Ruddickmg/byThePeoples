use crate::{utilities::hash, model, repository, Result};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UpdateResults {
    Success(model::Credentials),
    NotFound,
    Suspended,
    Unauthorized,
}

pub async fn update<
    L: repository::LoginHistory,
    C: repository::Credentials,
>(
    credentials: &C,
    login_history: &L,
    auth_details: &model::EmailRequest,
    request: &model::CredentialsRequest,
) -> Result<UpdateResults> {
    let model::CredentialsRequest {
        name,
        email,
        password,
    }: &model::CredentialsRequest = request;
    if let Some(stored_credentials) = credentials.by_email(&auth_details.email).await? {
        if stored_credentials.suspended()? {
            Ok(UpdateResults::Suspended)
        } else {
            if stored_credentials.password_matches(&auth_details.password)? {
                Ok(UpdateResults::Success(
                    credentials
                        .update_credentials(&model::Credentials {
                            name: name.as_ref().map_or(stored_credentials.name, String::from),
                            email: email
                                .as_ref()
                                .map_or(stored_credentials.email, String::from),
                            hash: match &password {
                                Some(p) => hash::generate(p)?,
                                None => stored_credentials.hash,
                            },
                            ..stored_credentials
                        })
                        .await?,
                ))
            } else {
                login_history.suspend(&stored_credentials.id).await?;
                Ok(UpdateResults::Unauthorized)
            }
        }
    } else {
        Ok(UpdateResults::NotFound)
    }
}

#[cfg(test)]
mod credentials_update_test {
    use super::*;
    use crate::utilities::{
        test::fake,
        hash,
    };
    use actix_rt;
    use std::time::SystemTime;

    #[actix_rt::test]
    async fn returns_not_found_when_a_record_does_not_exist() {
        let update_request = fake::credentials_request();
        let auth_request = fake::email_request();
        let mut state = fake::service_state();
        state.credentials.by_email.returns(None);
        let result = update(
            &state.credentials,
            &state.login_history,
            &auth_request,
            &update_request,
        )
        .await
        .unwrap();
        assert_eq!(result, UpdateResults::NotFound);
    }

    #[actix_rt::test]
    async fn returns_suspended_if_the_user_has_been_suspended() {
        let mut credentials = fake::credentials();
        let update_request = fake::credentials_request();
        let auth_request = fake::email_request();
        let mut state = fake::service_state();
        credentials.locked_at = Some(SystemTime::now());
        state.credentials.by_email.returns(Some(credentials));
        let result = update(
            &state.credentials,
            &state.login_history,
            &auth_request,
            &update_request,
        )
        .await
        .unwrap();
        assert_eq!(result, UpdateResults::Suspended);
    }

    #[actix_rt::test]
    async fn returns_unauthorized_if_credentials_dont_match() {
        let credentials = fake::credentials();
        let update_request = fake::credentials_request();
        let auth_request = fake::email_request();
        let mut state = fake::service_state();
        state.credentials.by_email.returns(Some(credentials));
        state.login_history.suspend.returns(());
        let result = update(
            &state.credentials,
            &state.login_history,
            &auth_request,
            &update_request,
        )
        .await
        .unwrap();
        assert_eq!(result, UpdateResults::Unauthorized);
    }

    #[actix_rt::test]
    async fn calls_suspend_if_credentials_dont_match() {
        let credentials = fake::credentials();
        let update_request = fake::credentials_request();
        let auth_request = fake::email_request();
        let mut state = fake::service_state();
        state.credentials.by_email.returns(Some(credentials));
        state.login_history.suspend.returns(());
        let result = update(
            &state.credentials,
            &state.login_history,
            &auth_request,
            &update_request,
        )
        .await
        .unwrap();
        assert_eq!(result, UpdateResults::Unauthorized);
    }

    #[actix_rt::test]
    async fn returns_success_if_credentials_match() {
        let mut credentials = fake::credentials();
        let update_request = fake::credentials_request();
        let auth_request = fake::email_request();
        let mut state = fake::service_state();
        credentials.hash = hash::generate(&auth_request.password).unwrap();
        state
            .credentials
            .by_email
            .returns(Some(credentials.clone()));
        state
            .credentials
            .update_credentials
            .returns(credentials.clone());
        let result = update(
            &state.credentials,
            &state.login_history,
            &auth_request,
            &update_request,
        )
        .await
        .unwrap();
        match result {
            UpdateResults::Success(_) => assert!(true),
            _ => assert!(false),
        };
    }

    #[actix_rt::test]
    async fn calls_update_credentials_if_credentials_match() {
        let mut credentials = fake::credentials();
        let update_request = fake::credentials_request();
        let auth_request = fake::email_request();
        let mut state = fake::service_state();
        credentials.hash = hash::generate(&auth_request.password).unwrap();
        state
            .credentials
            .by_email
            .returns(Some(credentials.clone()));
        state
            .credentials
            .update_credentials
            .returns(credentials.clone());
        update(
            &state.credentials,
            &state.login_history,
            &auth_request,
            &update_request,
        )
        .await
        .unwrap();
        assert_eq!(state.credentials.update_credentials.times_called(), 1);
    }
}
