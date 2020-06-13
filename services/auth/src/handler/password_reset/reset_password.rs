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
        Ok(result) => match result {
            password_reset::ResetResult::WeakPassword(problems) => serde_json::to_string(&problems)
                .map_or(HttpResponse::InternalServerError().finish(), | json | {
                    HttpResponse::Forbidden().json(&json)
                }),
            password_reset::ResetResult::Expired => HttpResponse::Gone().finish(),
            _ => HttpResponse::Accepted().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utilities::{test::fake, hash}, error::Error, configuration::PASSWORD_RESET_TIME_PERIOD};
    use std::{time::{SystemTime, Duration}, ops::Sub};
    use actix_rt;

    #[actix_rt::test]
    async fn returns_gone_if_the_reset_request_has_expired() {
        let credentials = fake::credentials();
        let request = fake::password_reset_data();
        let mut reset_record = fake::password_reset_request();
        let mut state = fake::service_state();
        reset_record.created_at = SystemTime::now().sub(Duration::from_secs(PASSWORD_RESET_TIME_PERIOD));
        state.reset_request.by_id.returns(Some(reset_record));
        state.credentials.update_password_hash.returns(credentials.clone());
        let result = reset_password(web::Data::new(state), web::Json(request))
            .await;
        assert_eq!(result.status(), status_codes::GONE);
    }

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

    #[actix_rt::test]
    async fn returns_forbidden_when_a_password_is_too_weak() {
        let credentials = fake::credentials();
        let mut reset_record = fake::password_reset_request();
        let mut request = fake::password_reset_data();
        let mut state = fake::service_state();
        request.password = fake::weak_password();
        reset_record.reset_token = hash::generate(&request.reset_token).unwrap();
        state.reset_request.by_id.returns(Some(reset_record));
        state.credentials.update_password_hash.returns(credentials.clone());
        let result = reset_password(web::Data::new(state), web::Json(request))
            .await;
        assert_eq!(result.status(), status_codes::FORBIDDEN);
    }
}
