use crate::{controller::password, model, repository, Error};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeleteResults {
    Success,
    NotFound,
    Suspended,
    Unauthorized,
}

pub async fn delete<
    L: repository::LoginHistory,
    C: repository::Credentials,
>(
    credentials: &C,
    login_history: &L,
    request: &model::EmailRequest,
) -> Result<DeleteResults, Error> {
    let model::EmailRequest { password, email }: &model::EmailRequest = request;
    if let Some(stored_credentials) = credentials.by_email(&email).await? {
        if stored_credentials.suspended()? {
            Ok(DeleteResults::Suspended)
        } else {
            if password::authenticate(&password, &stored_credentials.hash)? {
                credentials.mark_as_deleted_by_email(&email).await?;
                Ok(DeleteResults::Success)
            } else {
                login_history.suspend(&stored_credentials.id).await?;
                Ok(DeleteResults::Unauthorized)
            }
        }
    } else {
        Ok(DeleteResults::NotFound)
    }
}

#[cfg(test)]
mod credentials_delete_test {
    use super::*;
    use crate::utilities::test::fake;
    use actix_rt;
    use std::time::SystemTime;

    #[actix_rt::test]
    async fn returns_not_found_if_no_record_us_found() {
        let request = fake::email_request();
        let mut state = fake::service_state();
        state.credentials.by_email.returns(None);
        let result = delete(&state.credentials, &state.login_history, &request)
            .await
            .unwrap();
        assert_eq!(result, DeleteResults::NotFound);
    }

    #[actix_rt::test]
    async fn returns_suspended_if_a_record_was_suspended() {
        let request = fake::email_request();
        let mut credentials = fake::credentials();
        let mut state = fake::service_state();
        credentials.locked_at = Some(SystemTime::now());
        state.credentials.by_email.returns(Some(credentials));
        let result = delete(&state.credentials, &state.login_history, &request)
            .await
            .unwrap();
        assert_eq!(result, DeleteResults::Suspended);
    }

    #[actix_rt::test]
    async fn returns_unauthorized_if_credentials_dont_match() {
        let request = fake::email_request();
        let credentials = fake::credentials();
        let mut state = fake::service_state();
        state.credentials.by_email.returns(Some(credentials));
        state.login_history.suspend.returns(());
        let result = delete(&state.credentials, &state.login_history, &request)
            .await
            .unwrap();
        assert_eq!(result, DeleteResults::Unauthorized);
    }

    #[actix_rt::test]
    async fn calls_suspend_if_credentials_dont_match() {
        let request = fake::email_request();
        let credentials = fake::credentials();
        let mut state = fake::service_state();
        state.credentials.by_email.returns(Some(credentials));
        state.login_history.suspend.returns(());
        delete(&state.credentials, &state.login_history, &request)
            .await
            .unwrap();
        assert_eq!(state.login_history.suspend.times_called(), 1);
    }

    #[actix_rt::test]
    async fn returns_success_if_credentials_match() {
        let request = fake::email_request();
        let mut credentials = fake::credentials();
        let mut state = fake::service_state();
        credentials.hash = password::hash_password(&request.password).unwrap();
        state.credentials.by_email.returns(Some(credentials));
        state.credentials.mark_as_deleted_by_email.returns(1);
        let result = delete(&state.credentials, &state.login_history, &request)
            .await
            .unwrap();
        assert_eq!(result, DeleteResults::Success);
    }

    #[actix_rt::test]
    async fn calls_mark_as_deleted_if_credentials_match() {
        let request = fake::email_request();
        let mut credentials = fake::credentials();
        let mut state = fake::service_state();
        credentials.hash = password::hash_password(&request.password).unwrap();
        state.credentials.by_email.returns(Some(credentials));
        state.credentials.mark_as_deleted_by_email.returns(1);
        delete(&state.credentials, &state.login_history, &request)
            .await
            .unwrap();
        assert_eq!(state.credentials.mark_as_deleted_by_email.times_called(), 1);
    }
}
