use crate::{
    controller::{authorization, jwt},
    model, repository,
};
use actix_web::{web, HttpResponse};

pub async fn authenticate_credentials<
    T: model::Database,
    L: repository::LoginHistory<T>,
    C: repository::Credentials<T>,
>(
    state: web::Data<model::ServiceState<T, L, C>>,
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
    use crate::utilities::test::fake;
    use actix_rt;

    #[actix_rt::test]
    async fn returns_okay_on_successful_authentication() {}

    #[actix_rt::test]
    async fn sets_auth_header_on_successful_authentication() {}

    #[actix_rt::test]
    async fn returns_unauthorized_on_failed_authentication() {}

    #[actix_rt::test]
    async fn does_not_set_auth_header_on_failed_authentication() {}

    #[actix_rt::test]
    async fn returns_internal_server_error_on_unexpected_error() {}

    #[actix_rt::test]
    async fn does_not_set_auth_header_on_unexpected_error() {}
}
