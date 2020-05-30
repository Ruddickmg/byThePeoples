use actix_web::{web, HttpResponse};
use crate::{
    repository,
    controller::password_reset,
    model,
};

pub async fn reset_password<L, C, R>(
    state: web::Data<model::ServiceState<L, C, R>>,
    json: web::Json<model::ResetConfirmation>,
) -> HttpResponse
    where
        L: repository::LoginHistory,
        C: repository::Credentials,
        R: repository::PasswordResetRequest
{
    let request = model::ResetConfirmation::from(json);
    match password_reset::reset_password(&state.reset_request, &state.credentials, &request).await {
        Ok(_) => HttpResponse::Accepted(),
        Err(_) => HttpResponse::InternalServerError()
    }.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utilities::{test::fake, hash}, error::Error};
    use actix_rt;

    #[actix_rt::test]
    async fn returns_internal_server_error_when_an_error_occurs() {
        let error = Error::InternalServerError(String::from("Testing"));
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        state.reset_request.by_id.throws_error(error.clone());
        let result = reset_password(web::Data::new(state), web::Json(request))
            .await;
        assert_eq!(result.status(), status_codes::INTERNAL_SERVER_ERROR);
    }

    #[actix_rt::test]
    async fn returns_accepted_when_no_matching_request_is_found() {
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        state.reset_request.by_id.returns(None);
        let result = reset_password(web::Data::new(state), web::Json(request))
            .await;
        assert_eq!(result.status(), status_codes::ACCEPTED);
    }

    #[actix_rt::test]
    async fn returns_accepted_when_no_an_invalid_token_is_received() {
        let reset_record = fake::password_reset_request();
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        state.reset_request.by_id.returns(Some(reset_record));
        let result = reset_password(web::Data::new(state), web::Json(request))
            .await;
        assert_eq!(result.status(), status_codes::ACCEPTED);
    }

    #[actix_rt::test]
    async fn returns_accepted_on_successful_password_reset() {
        let credentials = fake::credentials();
        let mut reset_record = fake::password_reset_request();
        let request = fake::password_reset_data();
        let mut state = fake::service_state();
        reset_record.reset_token = hash::generate(&request.reset_token).unwrap();
        state.reset_request.by_id.returns(Some(reset_record));
        state.credentials.update_password_hash.returns(credentials.clone());
        let result = reset_password(web::Data::new(state), web::Json(request))
            .await;
        assert_eq!(result.status(), status_codes::ACCEPTED);
    }
}
