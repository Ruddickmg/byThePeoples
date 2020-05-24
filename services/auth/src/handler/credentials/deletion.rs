use crate::{controller::credentials, model, repository};
use actix_web::{web, HttpResponse};

pub async fn delete_credentials<
    L: repository::LoginHistory,
    C: repository::Credentials,
>(
    state: web::Data<model::ServiceState<L, C>>,
    json: web::Json<model::EmailRequest>,
) -> HttpResponse {
    let user_credentials = model::EmailRequest::from(json);
    match credentials::delete(&state.credentials, &state.login_history, &user_credentials).await {
        Ok(deletion) => match deletion {
            credentials::DeleteResults::Success => HttpResponse::Accepted(),
            _ => HttpResponse::Unauthorized(),
        },
        Err(_) => HttpResponse::InternalServerError(),
    }
    .finish()
}

#[cfg(test)]
mod delete_credentials_handler_test {
    use super::*;
    use crate::{utilities::{test::fake, hash}, Error};
    use actix_rt;
    use actix_web::web;

    #[actix_rt::test]
    async fn returns_accepted_on_successful_deletion() {
        let mut state = fake::service_state();
        let request = fake::email_request();
        let mut record = fake::credentials();
        record.hash = hash::generate(&request.password).unwrap();
        state.credentials.by_email.returns(Some(record.clone()));
        state.credentials.mark_as_deleted_by_email.returns(1);
        let result = delete_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::ACCEPTED);
    }

    #[actix_rt::test]
    async fn returns_unauthorized_on_failed_authentication() {
        let mut state = fake::service_state();
        let request = fake::email_request();
        state.credentials.by_email.returns(None);
        let result = delete_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn returns_internal_server_error_on_unexpected_error() {
        let error = Error::InternalServerError("testing".to_string());
        let mut state = fake::service_state();
        let request = fake::email_request();
        state.credentials.by_email.throws_error(error);
        let result = delete_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::INTERNAL_SERVER_ERROR);
    }
}
