use crate::{repository, Result, model, utilities::hash};

pub async fn request_password_reset<R: repository::PasswordResetRequest>(
    reset_request: &R,
    email: &str,
) -> Result<model::ResetToken> {
    reset_request.generate(email).await?
        .map_or_else(|| hash::generate(hash::token().as_ref())
        .map_or_else(| error | Err(error), | hashed | Ok(model::ResetToken::new(
            hash::token().as_ref(),
            &hashed,
        ))), | record | Ok(model::ResetToken::new(&record.id, &record.reset_token)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        utilities::test::fake,
        error,
    };
    use actix_rt;
    use std::any::Any;

    #[actix_rt::test]
    async fn returns_the_request_record_on_successful_generation() {
        let mut state = fake::service_state();
        let email = "test@testing.com";
        let reset_request = fake::password_reset_request();
        let reset_token = model::ResetToken::new(&reset_request.id, &reset_request.reset_token);
        state.reset_request.generate.returns(Some(reset_request));
        let result = request_password_reset(&state.reset_request, email)
            .await.unwrap();
        assert_eq!(result, reset_token);
    }

    #[actix_rt::test]
    async fn returns_dummy_request_record_if_no_matching_user_was_found() {
        let mut state = fake::service_state();
        let request = fake::password_reset_request();
        let email = "test@testing.com";
        let reset_token = model::ResetToken::new(&request.id, &request.reset_token);
        state.reset_request.generate.returns(None);
        let result = request_password_reset(&state.reset_request, email)
            .await.unwrap();
        assert_eq!(reset_token.type_id(), result.type_id());
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