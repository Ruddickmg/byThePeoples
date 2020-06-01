use crate::{repository, Result, utilities::{hash, password}, model};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResetResult {
    Success(model::Credentials),
    WeakPassword(password::PasswordIssues),
    InvalidToken,
    NotFound,
    Expired,
}

pub async fn reset_password<R: repository::PasswordResetRequest, C: repository::Credentials>(
    reset_request: &R,
    credentials: &C,
    data: &model::ResetConfirmation,
) -> Result<ResetResult> {
    Ok(if let Some(request) = reset_request.by_id(&data.id).await? {
        if request.expired()? {
            ResetResult::Expired
        } else {
            if let password::Strength::Weak(problems) = password::strength(&request.name, &request.email, &data.password)? {
                ResetResult::WeakPassword(problems)
            } else {
                if hash::authenticate(&data.reset_token, &request.reset_token)? {
                    let hashed_password = hash::generate(&data.password)?;
                    ResetResult::Success(credentials.update_password_hash(&request.user_id, &hashed_password).await?)
                } else {
                    ResetResult::InvalidToken
                }
            }
        }
    } else {
        ResetResult::NotFound
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt;
    use crate::{utilities::test::fake, error::Error};

    #[actix_rt::test]
    async fn returns_invalid_token_when_the_reset_token_doesnt_match() {
        let reset_record = fake::password_reset_request();
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        state.reset_request.by_id.returns(Some(reset_record));
        let result = reset_password(&state.reset_request, &state.credentials, &request)
            .await.unwrap();
        assert_eq!(result, ResetResult::InvalidToken);
    }

    #[actix_rt::test]
    async fn returns_not_found_when_no_record_is_found_by_its_id() {
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        state.reset_request.by_id.returns(None);
        let result = reset_password(&state.reset_request, &state.credentials, &request)
            .await.unwrap();
        assert_eq!(result, ResetResult::NotFound);
    }

    #[actix_rt::test]
    async fn returns_success_when_credentials_are_found_and_valid() {
        let credentials = fake::credentials();
        let mut reset_record = fake::password_reset_request();
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        reset_record.reset_token = hash::generate(&request.reset_token).unwrap();
        state.reset_request.by_id.returns(Some(reset_record));
        state.credentials.update_password_hash.returns(credentials.clone());
        let result = reset_password(&state.reset_request, &state.credentials, &request)
            .await.unwrap();
        assert_eq!(result, ResetResult::Success(credentials.clone()));
    }

    #[actix_rt::test]
    async fn returns_updated_credentials_when_reset_is_successful() {
        let credentials = fake::credentials();
        let mut reset_record = fake::password_reset_request();
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        reset_record.reset_token = hash::generate(&request.reset_token).unwrap();
        state.reset_request.by_id.returns(Some(reset_record));
        state.credentials.update_password_hash.returns(credentials.clone());
        let result = reset_password(&state.reset_request, &state.credentials, &request)
            .await.unwrap();
        if let ResetResult::Success(reset_record) = result.clone() {
            assert_eq!(reset_record, credentials.clone());
        } else {
            panic!(format!("Invalid results: {:#?}", result.clone()));
        }
    }

    #[actix_rt::test]
    async fn returns_an_error_when_one_occurs() {
        let error = Error::InternalServerError(String::from("Cool test"));
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        state.reset_request.by_id.throws_error(error.clone());
        let result = reset_password(&state.reset_request, &state.credentials, &request)
            .await.err().unwrap();
        assert_eq!(result.to_string(), error.clone().to_string());
    }

    #[actix_rt::test]
    async fn returns_weak_password_if_the_password_is_weak() {
        let credentials = fake::credentials();
        let mut reset_record = fake::password_reset_request();
        let mut request = fake::password_reset_data();
        let mut state = fake::service_state();
        request.password = fake::weak_password();
        reset_record.reset_token = hash::generate(&request.reset_token).unwrap();
        state.reset_request.by_id.returns(Some(reset_record));
        state.credentials.update_password_hash.returns(credentials.clone());
        let result = reset_password(&state.reset_request, &state.credentials, &request)
            .await.unwrap();
        match result {
            ResetResult::WeakPassword(_) => assert!(true),
            _ => panic!(format!("Expected weak password result, found: {:#?}", result)),
        }
    }
}