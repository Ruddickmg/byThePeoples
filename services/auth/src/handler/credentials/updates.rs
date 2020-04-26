use crate::constants::SUSPENDED_ACCOUNT_MESSAGE;
use crate::{
    controller::{credentials, jwt},
    model,
};
use actix_web::{web, HttpResponse};

pub async fn update(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::UpdateCredentials>,
) -> HttpResponse {
    let update_credentials = model::UpdateCredentials::from(json);
    let model::UpdateCredentials {
        auth,
        credentials: updates,
    } = update_credentials;
    if let Ok(status) = credentials::update(&state.db, &auth, &updates).await {
        match status {
            credentials::UpdateResults::Success(credentials) => {
                jwt::set_token(HttpResponse::Ok(), credentials)
                    .unwrap_or(HttpResponse::InternalServerError().finish())
            }
            credentials::UpdateResults::Suspended => {
                HttpResponse::Forbidden().body(SUSPENDED_ACCOUNT_MESSAGE)
            }
            _ => HttpResponse::Unauthorized().finish(),
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[cfg(test)]
mod update_credentials_test {
    use super::*;
    use crate::{controller, model, utilities::test as test_helper};
    use actix_web::{http, test, FromRequest};

    #[actix_rt::test]
    async fn returns_okay_if_the_update_was_successful() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let helper = test_helper::Helper::new().await.unwrap();
        let (name, email, password) = test_helper::fake_credentials();
        let (name2, email2, password2) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let auth_credentials = model::EmailRequest::new(&email, &password);
        let update_credentials = model::CredentialsRequest::new(
            &Some(name2.clone()),
            &Some(email2.clone()),
            &Some(password2.clone()),
        );
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = update(request_state, json).await;
        helper.delete_credentials_by_name(&name2).await;
        assert_eq!(resp.status(), status_codes::OKAY);
    }

    #[actix_rt::test]
    async fn sets_updated_auth_token_on_successful_response() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let helper = test_helper::Helper::new().await.unwrap();
        let (name, email, password) = test_helper::fake_credentials();
        let (name2, email2, password2) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let auth_credentials = model::EmailRequest::new(&email, &password);
        let update_credentials = model::CredentialsRequest::new(
            &Some(name2.clone()),
            &Some(email2.clone()),
            &Some(password2.clone()),
        );
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = update(request_state, json).await;
        helper.delete_credentials_by_name(&name2).await;
        assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn updates_a_users_name() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let helper = test_helper::Helper::new().await.unwrap();
        let (name, email, password) = test_helper::fake_credentials();
        let (name2, email2, password2) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let auth_credentials = model::EmailRequest::new(&email, &password);
        let update_credentials = model::CredentialsRequest::new(&Some(name2.clone()), &None, &None);
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        update(request_state, json).await;
        let stored_credentials = helper
            .get_credentials_by_name(&name2)
            .await
            .unwrap()
            .unwrap();
        helper.delete_credentials_by_name(&name2).await;
        assert_eq!(&stored_credentials.name, &name2);
        assert_eq!(&stored_credentials.email, &email);
        assert_eq!(&stored_credentials.hash, &hashed_password);
    }

    #[actix_rt::test]
    async fn updates_a_users_password() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let helper = test_helper::Helper::new().await.unwrap();
        let (name, email, password) = test_helper::fake_credentials();
        let (name2, email2, password2) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let hashed_password2 = controller::password::hash_password(&password2).unwrap();
        let auth_credentials = model::EmailRequest::new(&email, &password);
        let update_credentials =
            model::CredentialsRequest::new(&None, &None, &Some(password2.clone()));
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        update(request_state, json).await;
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper.delete_credentials_by_name(&name2).await;
        assert_eq!(&stored_credentials.email, &email);
        assert_eq!(&stored_credentials.name, &name);
        assert!(controller::password::authenticate(&password2, &stored_credentials.hash).unwrap());
    }

    #[actix_rt::test]
    async fn updates_a_users_email() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let helper = test_helper::Helper::new().await.unwrap();
        let (name, email, password) = test_helper::fake_credentials();
        let (_name2, email2, password2) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let hashed_password2 = controller::password::hash_password(&password2).unwrap();
        let auth_credentials = model::EmailRequest::new(&email, &password);
        let update_credentials =
            model::CredentialsRequest::new(&None, &Some(email2.clone()), &None);
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        update(request_state, json).await;
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(&stored_credentials.email, &email2);
        assert_eq!(&stored_credentials.hash, &hashed_password);
        assert_eq!(&stored_credentials.name, &name);
    }

    #[actix_rt::test]
    async fn returns_unauthorized_if_auth_credentials_are_invalid() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let helper = test_helper::Helper::new().await.unwrap();
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
        let update_credentials = model::CredentialsRequest::new(&None, &None, &None);
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = update(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_token_if_unauthorized() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let helper = test_helper::Helper::new().await.unwrap();
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
        let update_credentials = model::CredentialsRequest::new(&None, &None, &None);
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = update(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_unauthorized_if_no_associated_record_exists() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let helper = test_helper::Helper::new().await.unwrap();
        let (_name, email, password) = test_helper::fake_credentials();
        let auth_credentials = model::EmailRequest::new(&email, &password);
        let update_credentials = model::CredentialsRequest::new(&None, &None, &None);
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = update(request_state, json).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn doest_not_set_auth_token_if_not_found() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (_name, email, password) = test_helper::fake_credentials();
        let auth_credentials = model::EmailRequest::new(&email, &password);
        let update_credentials = model::CredentialsRequest::new(&None, &None, &None);
        let data = model::UpdateCredentials::new(&auth_credentials, &update_credentials);
        let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
        let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = update(request_state, json).await;
        assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    // TODO test suspension

    #[actix_rt::test]
    async fn errors_with_forbidden_if_a_user_has_been_suspended() {}

    #[actix_rt::test]
    async fn suspends_a_user_if_they_have_exceeded_the_allowed_failed_login_attempts() {}

    #[actix_rt::test]
    async fn deletes_the_login_history_once_a_user_has_been_suspended() {}

    #[actix_rt::test]
    async fn deletes_the_failed_login_history_if_a_user_successfully_logs_in() {}

    #[actix_rt::test]
    async fn creates_a_log_of_failed_login_attempts() {}
}
