use crate::{
    controller::{authorization, jwt},
    model, repository,
};
use actix_web::{web, HttpResponse};

pub async fn authenticate_credentials<
    L: repository::LoginHistory,
    C: repository::Credentials,
>(
    state: web::Data<model::ServiceState<L, C>>,
    json: web::Json<model::NameRequest>,
) -> HttpResponse {
    let user_credentials = model::NameRequest::from(json);
    match authorization::authorize(&user_credentials, &state.credentials, &state.login_history)
        .await
    {
        Ok(stored_credentials) => match stored_credentials {
            authorization::Results::Valid(credentials) => {
                jwt::set_token(HttpResponse::Ok(), credentials)
                    .unwrap_or(HttpResponse::InternalServerError().finish())
            }
            _ => HttpResponse::Unauthorized().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod verification_handler_test {
    use super::*;
    use crate::{controller::password, utilities::test::fake, Error};
    use actix_rt;
    use actix_web::{http, web};

    #[actix_rt::test]
    async fn returns_okay_on_successful_authentication() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        let mut record = fake::credentials();
        record.hash = password::hash_password(&request.password).unwrap();
        state.credentials.by_name.returns(Some(record.clone()));
        let result = authenticate_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::OKAY);
    }

    #[actix_rt::test]
    async fn sets_auth_header_on_successful_authentication() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        let mut record = fake::credentials();
        record.hash = password::hash_password(&request.password).unwrap();
        state.credentials.by_name.returns(Some(record.clone()));
        let result = authenticate_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(result.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_unauthorized_on_failed_authentication() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        state.credentials.by_name.returns(None);
        let result = authenticate_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_header_on_failed_authentication() {
        let mut state = fake::service_state();
        let request = fake::name_request();
        state.credentials.by_name.returns(None);
        let result = authenticate_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(!result.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_internal_server_error_on_unexpected_error() {
        let error = Error::InternalServerError("testing".to_string());
        let mut state = fake::service_state();
        let request = fake::name_request();
        state.credentials.by_name.throws_error(error);
        let result = authenticate_credentials(web::Data::new(state), web::Json(request)).await;
        assert_eq!(result.status(), status_codes::INTERNAL_SERVER_ERROR);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_header_on_unexpected_error() {
        let error = Error::InternalServerError("testing".to_string());
        let mut state = fake::service_state();
        let request = fake::name_request();
        state.credentials.by_name.throws_error(error);
        let result = authenticate_credentials(web::Data::new(state), web::Json(request)).await;
        assert!(!result.headers().contains_key(http::header::AUTHORIZATION));
    }
}
