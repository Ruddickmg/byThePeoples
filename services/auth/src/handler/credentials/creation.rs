use crate::{
    controller::{credentials, jwt},
    model, repository,
};
use actix_web::{web, HttpResponse};

pub async fn save_credentials<
    L: repository::LoginHistory,
    C: repository::Credentials,
>(
    state: web::Data<model::ServiceState<L, C>>,
    json: web::Json<model::FullRequest>,
) -> HttpResponse {
    let user_credentials = model::FullRequest::from(json);
    match credentials::create(&state.credentials, &user_credentials).await {
        Ok(result) => match result {
            credentials::SaveResults::Conflict => HttpResponse::Conflict().finish(),
            credentials::SaveResults::WeakPassword(problems) => serde_json::to_string(&problems)
                .map_or(HttpResponse::InternalServerError().finish(), |json| {
                    HttpResponse::Forbidden().json2(&json)
                }),
            credentials::SaveResults::Success(stored_credentials) => {
                jwt::set_token(HttpResponse::Created(), stored_credentials)
                    .unwrap_or(HttpResponse::InternalServerError().finish())
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod save_credentials_handler_test {
    use super::*;
    use crate::{repository, utilities::test::fake, Error};
    use actix_rt;
    use actix_web::{http, web};

    const WEAK_PASSWORD: &str = "password";

    #[actix_rt::test]
    async fn returns_created_on_successful_creation() {
        let mut state = fake::service_state();
        let request = fake::full_request();
        let record = fake::credentials();
        state.credentials.by_name.returns(None);
        state
            .credentials
            .get_status
            .returns(repository::credentials::Status::None);
        state.credentials.save_credentials.returns(record);
        let result = save_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::CREATED);
    }

    #[actix_rt::test]
    async fn sets_auth_header_on_successful_authentication() {
        let mut state = fake::service_state();
        let request = fake::full_request();
        let record = fake::credentials();
        state.credentials.by_name.returns(None);
        state
            .credentials
            .get_status
            .returns(repository::credentials::Status::None);
        state.credentials.save_credentials.returns(record);
        let result = save_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(result.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_forbidden_when_a_password_is_too_weak() {
        let state = fake::service_state();
        let mut request = fake::full_request();
        request.password = WEAK_PASSWORD.to_string();
        let result = save_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::FORBIDDEN);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_header_when_password_is_too_weak() {
        let state = fake::service_state();
        let mut request = fake::full_request();
        request.password = WEAK_PASSWORD.to_string();
        let result = save_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(!result.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_conflict_when_a_matching_record_exists() {
        let mut state = fake::service_state();
        let request = fake::full_request();
        state
            .credentials
            .get_status
            .returns(repository::credentials::Status::Exists);
        let result = save_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::CONFLICT);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_header_on_conflict() {
        let mut state = fake::service_state();
        let request = fake::full_request();
        state
            .credentials
            .get_status
            .returns(repository::credentials::Status::Exists);
        let result = save_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(!result.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_internal_server_error_on_unexpected_error() {
        let error = Error::InternalServerError("testing".to_string());
        let mut state = fake::service_state();
        let request = fake::full_request();
        state.credentials.get_status.throws_error(error);
        let result = save_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::INTERNAL_SERVER_ERROR);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_header_on_unexpected_error() {
        let error = Error::InternalServerError("testing".to_string());
        let mut state = fake::service_state();
        let request = fake::full_request();
        state.credentials.get_status.throws_error(error);
        let result = save_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(!result.headers().contains_key(http::header::AUTHORIZATION));
    }
}
