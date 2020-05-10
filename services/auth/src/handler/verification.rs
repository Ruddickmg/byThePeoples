use crate::{
    controller::{authorization, jwt},
    model,
};
use actix_web::{web, HttpResponse};

pub async fn authenticate_credentials<T: model::Database>(
    state: web::Data<model::ServiceState<T>>,
    json: web::Json<model::NameRequest>,
) -> HttpResponse {
    let user_credentials = model::NameRequest::from(json);
    match authorization::authorize(&user_credentials, &state.db).await {
        Ok(stored_credentials) => match stored_credentials {
            authorization::Results::Valid(credentials) => {
                match jwt::set_token(HttpResponse::Ok(), credentials) {
                    Ok(authenticated_response) => authenticated_response,
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            _ => HttpResponse::Unauthorized().finish(),
        },
        Err(error) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;
    use crate::{
        configuration::{
            database::TEST_DATABASE_CONFIG, ACCOUNT_LOCK_DURATION_IN_SECONDS,
            ALLOWED_FAILED_LOGIN_ATTEMPTS,
        },
        controller, model,
        utilities::test as test_helper,
    };
    use actix_web::{http, test, FromRequest};
    use std::{
        ops::Sub,
        time::{Duration, SystemTime},
    };

    #[actix_rt::test]
    async fn authenticate_credentials_success_status() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::NameRequest::new(&name, &password);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::OKAY);
    }

    #[actix_rt::test]
    async fn authenticate_credentials_sets_auth_header() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::NameRequest::new(&name, &password);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn errors_with_unauthorized_if_no_record_exists() {
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, _email, password) = test_helper::fake_credentials();
        let request_data = model::NameRequest::new(&name, &password);
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn errors_with_unauthorized_if_passwords_do_not_match() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let data = model::FullRequest::new(&name, &hashed_password, &email);
        helper.add_credentials(&data).await;
        let request_data = model::NameRequest::new(&name, "invalid password");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn returns_unauthorized_if_a_user_has_been_suspended() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let data = model::FullRequest::new(&name, &hashed_password, &email);
        helper.add_credentials(&data).await;
        let request_data = model::NameRequest::new(&name, "invalid password");
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper.suspend_user(&stored_credentials.id).await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn suspends_a_user_if_they_have_exceeded_the_allowed_failed_login_attempts() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let data = model::FullRequest::new(&name, &hashed_password, &email);
        helper.add_credentials(&data).await;
        let request_data = model::NameRequest::new(&name, "invalid password");
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper
            .set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        authenticate_credentials(request_state, json).await;
        let user_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_ne!(user_credentials.locked_at, None);
    }

    #[actix_rt::test]
    async fn deletes_the_login_history_once_a_user_has_been_suspended() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let data = model::FullRequest::new(&name, &hashed_password, &email);
        helper.add_credentials(&data).await;
        let request_data = model::NameRequest::new(&name, "invalid password");
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper
            .set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        authenticate_credentials(request_state, json).await;
        let login_history = helper
            .get_login_history(&stored_credentials.id)
            .await
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(login_history.len(), 0);
    }

    #[actix_rt::test]
    async fn deletes_login_history_if_previous_login_failures_are_expired() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let data = model::FullRequest::new(&name, &hashed_password, &email);
        helper.add_credentials(&data).await;
        let request_data = model::NameRequest::new(&name, "invalid password");
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        let expired_timestamp =
            SystemTime::now().sub(Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS + 1));
        helper
            .set_login_history(&model::FailedLogin {
                user_id: stored_credentials.id,
                updated_at: expired_timestamp,
                created_at: expired_timestamp,
                attempts: ALLOWED_FAILED_LOGIN_ATTEMPTS,
            })
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        authenticate_credentials(request_state, json).await;
        let login_history = helper
            .get_login_history(&stored_credentials.id)
            .await
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(login_history.len(), 0);
    }

    #[actix_rt::test]
    async fn does_not_suspend_user_if_previous_login_failures_are_expired() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let data = model::FullRequest::new(&name, &hashed_password, &email);
        helper.add_credentials(&data).await;
        let request_data = model::NameRequest::new(&name, "invalid password");
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        let expired_timestamp =
            SystemTime::now().sub(Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS + 1));
        helper
            .set_login_history(&model::FailedLogin {
                user_id: stored_credentials.id,
                updated_at: expired_timestamp,
                created_at: expired_timestamp,
                attempts: ALLOWED_FAILED_LOGIN_ATTEMPTS,
            })
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        authenticate_credentials(request_state, json).await;
        let user_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(user_credentials.locked_at, None);
    }

    #[actix_rt::test]
    async fn creates_a_log_of_failed_login_attempts() {
        let helper = test_helper::Helper::new().await.unwrap();
        let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
            .await
            .unwrap();
        let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let data = model::FullRequest::new(&name, &hashed_password, &email);
        helper.add_credentials(&data).await;
        let request_data = model::NameRequest::new(&name, "invalid password");
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::NameRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        let login_history = helper
            .get_login_history(&stored_credentials.id)
            .await
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(login_history.len(), 1);
    }
}
