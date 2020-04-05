use crate::{
    controller::{credentials, jwt},
    model,
};
use actix_web::{web, HttpResponse};

pub async fn update(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::OptionUpdateRequest>,
) -> HttpResponse {
    if let model::UpdateRequest::Valid(credentials_update) = model::UpdateRequest::from(json) {
        let model::UpdateCredentials {
            auth,
            credentials: updates,
        }: model::UpdateCredentials = credentials_update;
        if let Ok(status) = credentials::update(&state.db, &auth, &updates).await {
            match status {
                credentials::UpdateResults::Success(credentials) => {
                    match jwt::set_token(HttpResponse::Ok(), credentials) {
                        Ok(authenticated_response) => authenticated_response,
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    }
                }
                credentials::UpdateResults::Unauthorized => HttpResponse::Unauthorized().finish(),
                credentials::UpdateResults::NotFound => HttpResponse::NotFound().finish(),
            }
        } else {
            HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[cfg(test)]
mod update_credentials_test {
    use super::*;
    use crate::{model, utilities::test as test_helper};
    use actix_web::{http, test, FromRequest};

    #[actix_rt::test]
    async fn returns_okay_if_the_update_was_successful() {
        assert_eq!(resp.status(), status_codes::OKAY);
    }

    #[actix_rt::test]
    async fn sets_updated_auth_token_on_successful_response() {}

    #[actix_rt::test]
    async fn updates_a_users_name() {}

    #[actix_rt::test]
    async fn updates_a_users_password() {}

    #[actix_rt::test]
    async fn updates_a_users_email() {}

    #[actix_rt::test]
    async fn returns_unauthorized_if_auth_credentials_are_invalid() {}

    #[actix_rt::text]
    async fn does_not_set_auth_token_if_unauthorized() {}

    #[actix_rt::test]
    async fn returns_not_found_if_no_associated_record_exists() {}

    #[actix_rt::test]
    async fn doest_not_set_auth_token_if_not_found() {}

    #[actix_rt::test]
    async fn returns_bad_request_if_update_details_are_missing() {}

    #[actix_rt::test]
    async fn returns_bad_request_if_auth_details_are_missing() {}

    #[async_rt::test]
    async fn returns_bad_request_if_auth_name_is_missing() {}

    #[async_rt::test]
    async fn returns_bad_request_if_auth_password_is_missing() {}

    #[async_rt::test]
    async fn does_not_set_auth_token_if_bad_request() {}
}
