use crate::{
    controller::credentials,
    utilities::jwt,
    repository,
    model,
};
use actix_web::{web, HttpResponse};

pub async fn update_credentials<
    L: repository::LoginHistory,
    C: repository::Credentials,
>(
    state: web::Data<model::ServiceState<L, C>>,
    json: web::Json<model::UpdateCredentials>,
) -> HttpResponse {
    let updated_credentials = model::UpdateCredentials::from(json);
    let model::UpdateCredentials {
        auth,
        credentials: updates,
    } = updated_credentials;
    match credentials::update(&state.credentials, &state.login_history, &auth, &updates).await {
        Ok(status) => match status {
            credentials::UpdateResults::Success(credentials) => {
                jwt::set_token(HttpResponse::Ok(), credentials)
                    .unwrap_or(HttpResponse::InternalServerError().finish())
            }
            _ => HttpResponse::Unauthorized().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod update_credentials_handler_test {
    use super::*;
    use crate::{utilities::{test::fake, hash}, Error};
    use actix_rt;
    use actix_web::{http, web};

    #[actix_rt::test]
    async fn returns_okay_on_successful_authentication() {
        let mut state = fake::service_state();
        let request = fake::update_credentials_request();
        let mut record = fake::credentials();
        record.hash = hash::generate(&request.auth.password).unwrap();
        state.credentials.by_email.returns(Some(record.clone()));
        state.credentials.update_credentials.returns(record.clone());
        let result = update_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::OKAY);
    }

    #[actix_rt::test]
    async fn sets_auth_header_on_successful_authentication() {
        let mut state = fake::service_state();
        let request = fake::update_credentials_request();
        let mut record = fake::credentials();
        record.hash = hash::generate(&request.auth.password).unwrap();
        state.credentials.by_email.returns(Some(record.clone()));
        state.credentials.update_credentials.returns(record.clone());
        let result = update_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(result.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_unauthorized_on_failed_authentication() {
        let mut state = fake::service_state();
        let request = fake::update_credentials_request();
        state.credentials.by_email.returns(None);
        let result = update_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_header_on_failed_authentication() {
        let mut state = fake::service_state();
        let request = fake::update_credentials_request();
        state.credentials.by_email.returns(None);
        let result = update_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(!result.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_internal_server_error_on_unexpected_error() {
        let error = Error::InternalServerError("testing".to_string());
        let mut state = fake::service_state();
        let request = fake::update_credentials_request();
        state.credentials.by_email.throws_error(error);
        let result = update_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::INTERNAL_SERVER_ERROR);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_header_on_unexpected_error() {
        let error = Error::InternalServerError("testing".to_string());
        let mut state = fake::service_state();
        let request = fake::update_credentials_request();
        state.credentials.by_email.throws_error(error);
        let result = update_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(!result.headers().contains_key(http::header::AUTHORIZATION));
    }
}
