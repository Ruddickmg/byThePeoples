use crate::{repository, Result, model, utilities::hash};
use std::time::SystemTime;

pub async fn request_password_reset<R: repository::PasswordResetRequest>(
    reset_request: &R,
    email: &str,
) -> Result<model::PasswordResetRequest> {
    Ok(reset_request.generate(email).await?.unwrap_or(model::PasswordResetRequest {
        id: hash::token(),
        reset_token: hash::token(),
        user_id: 0,
        created_at: SystemTime::now(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        utilities::test::fake,
        model,
        error,
    };
    use actix_rt;
    use std::time::SystemTime;

    #[actix_rt::test]
    async fn returns_the_request_token_on_successful_generation() {
        let mut state = fake::service_state();
        let email = "test@testing.com";
        let reset_request = model::PasswordResetRequest {
            id: "2432345".to_string(),
            reset_token: "2342534534".to_string(),
            user_id: 0,
            created_at: SystemTime::now(),
        };
        state.reset_request.generate.returns(Some(reset_request.clone()));
        let result = request_password_reset(&state.reset_request, email)
            .await.unwrap().unwrap();
        assert_eq!(result, reset_request.clone().reset_token);
    }

    #[actix_rt::test]
    async fn returns_none_if_no_matching_user_was_found() {
        let mut state = fake::service_state();
        let email = "test@testing.com";
        state.reset_request.generate.returns(None);
        let result = request_password_reset(&state.reset_request, email)
            .await.unwrap();
        assert_eq!(result, None);
    }

    #[actix_rt::test]
    async fn returns_an_error_if_something_went_wrong() {
        let mut state = fake::service_state();
        let email = "test@testing.com";
        let reset_error = error::Error::InternalServerError(String::from("testing123"));
        state.reset_request.generate.throws_error(reset_error.clone());
        let result = request_password_reset(&state.reset_request, email)
            .await.err().unwrap();
        assert_eq!(result.to_string(), reset_error.clone().to_string());
    }
}