use crate::{
    controller::{credentials, jwt},
    model, repository,
};
use actix_web::{web, HttpResponse};

pub async fn update_credentials<
    T: model::Database,
    L: repository::LoginHistory<T>,
    C: repository::Credentials<T>,
>(
    state: web::Data<model::ServiceState<T, L, C>>,
    json: web::Json<model::UpdateCredentials>,
) -> HttpResponse {
    let updated_credentials = model::UpdateCredentials::from(json);
    let model::UpdateCredentials {
        auth,
        credentials: updates,
    } = updated_credentials;
    if let Ok(status) =
        credentials::update(&state.credentials, &state.login_history, &auth, &updates).await
    {
        match status {
            credentials::UpdateResults::Success(credentials) => {
                jwt::set_token(HttpResponse::Ok(), credentials)
                    .unwrap_or(HttpResponse::InternalServerError().finish())
            }
            _ => HttpResponse::Unauthorized().finish(),
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
//
// #[cfg(test)]
// mod update_credentials_test {
//     use super::*;
//     use crate::{
//         configuration::{
//             database::TEST_DATABASE_CONFIG, ACCOUNT_LOCK_DURATION_IN_SECONDS,
//             ALLOWED_FAILED_LOGIN_ATTEMPTS,
//         },
//         controller, model,
//         utilities::test as test_helper,
//     };
//     use actix_web::{http, test, FromRequest};
//     use std::{
//         ops::Sub,
//         time::{Duration, SystemTime},
//     };
//
//     #[actix_rt::test]
//     async fn returns_okay_if_the_update_was_successful() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let (name2, email2, password2) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, &password);
//         let updated_credentials = model::CredentialsRequest::new(
//             &Some(name2.clone()),
//             &Some(email2.clone()),
//             &Some(password2.clone()),
//         );
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         let resp = update_credentials(request_state, json).await;
//         helper.delete_credentials_by_name(&name2).await;
//         assert_eq!(resp.status(), status_codes::OKAY);
//     }
//
//     #[actix_rt::test]
//     async fn sets_updated_auth_token_on_successful_response() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let (name2, email2, password2) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, &password);
//         let updated_credentials = model::CredentialsRequest::new(
//             &Some(name2.clone()),
//             &Some(email2.clone()),
//             &Some(password2.clone()),
//         );
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         let resp = update_credentials(request_state, json).await;
//         helper.delete_credentials_by_name(&name2).await;
//         assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
//     }
//
//     #[actix_rt::test]
//     async fn updates_a_users_name() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let (name2, email2, password2) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, &password);
//         let updated_credentials =
//             model::CredentialsRequest::new(&Some(name2.clone()), &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         update_credentials(request_state, json).await;
//         let stored_credentials = helper
//             .get_credentials_by_name(&name2)
//             .await
//             .unwrap()
//             .unwrap();
//         helper.delete_credentials_by_name(&name2).await;
//         assert_eq!(&stored_credentials.name, &name2);
//         assert_eq!(&stored_credentials.email, &email);
//         assert_eq!(&stored_credentials.hash, &hashed_password);
//     }
//
//     #[actix_rt::test]
//     async fn updates_a_users_password() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let (name2, email2, password2) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, &password);
//         let updated_credentials =
//             model::CredentialsRequest::new(&None, &None, &Some(password2.clone()));
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         update_credentials(request_state, json).await;
//         let stored_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         helper.delete_credentials_by_name(&name2).await;
//         assert_eq!(&stored_credentials.email, &email);
//         assert_eq!(&stored_credentials.name, &name);
//         assert!(controller::password::authenticate(&password2, &stored_credentials.hash).unwrap());
//     }
//
//     #[actix_rt::test]
//     async fn updates_a_users_email() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let (_name2, email2, password2) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, &password);
//         let updated_credentials =
//             model::CredentialsRequest::new(&None, &Some(email2.clone()), &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         update_credentials(request_state, json).await;
//         let stored_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         helper.delete_credentials_by_name(&name).await;
//         assert_eq!(&stored_credentials.email, &email2);
//         assert_eq!(&stored_credentials.hash, &hashed_password);
//         assert_eq!(&stored_credentials.name, &name);
//     }
//
//     #[actix_rt::test]
//     async fn returns_unauthorized_if_auth_credentials_are_invalid() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         let resp = update_credentials(request_state, json).await;
//         helper.delete_credentials_by_name(&name).await;
//         assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
//     }
//
//     #[actix_rt::test]
//     async fn does_not_set_auth_token_if_unauthorized() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         let resp = update_credentials(request_state, json).await;
//         helper.delete_credentials_by_name(&name).await;
//         assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
//     }
//
//     #[actix_rt::test]
//     async fn returns_unauthorized_if_no_associated_record_exists() {
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (_name, email, password) = test_helper::fake_credentials();
//         let auth_credentials = model::EmailRequest::new(&email, &password);
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         let resp = update_credentials(request_state, json).await;
//         assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
//     }
//
//     #[actix_rt::test]
//     async fn doest_not_set_auth_token_if_not_found() {
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (_name, email, password) = test_helper::fake_credentials();
//         let auth_credentials = model::EmailRequest::new(&email, &password);
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         let resp = update_credentials(request_state, json).await;
//         assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
//     }
//
//     #[actix_rt::test]
//     async fn returns_unauthorized_if_a_user_has_been_suspended() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, &password);
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let stored_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         helper.suspend_user(&stored_credentials.id).await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         let resp = update_credentials(request_state, json).await;
//         helper.delete_credentials_by_name(&name).await;
//         assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
//     }
//
//     #[actix_rt::test]
//     async fn suspends_a_user_if_they_have_exceeded_the_allowed_failed_update_attempts() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let stored_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         helper
//             .set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         update_credentials(request_state, json).await;
//         let user_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         helper.delete_credentials_by_name(&name).await;
//         assert_ne!(user_credentials.locked_at, None);
//     }
//
// #[actix_rt::test]
// async fn deletes_the_login_history_once_a_user_has_been_suspended() {
//     let helper = test_helper::Helper::new().await.unwrap();
//     let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//         .await
//         .unwrap();
//     let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//     let (name, email, password) = test_helper::fake_credentials();
//     let hashed_password = controller::password::hash_password(&password).unwrap();
//     let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
//     let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//     let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//     helper
//         .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//         .await;
//     let stored_credentials = helper
//         .get_credentials_by_name(&name)
//         .await
//         .unwrap()
//         .unwrap();
//     helper
//         .set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
//         .await;
//     let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//     let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//         .await
//         .unwrap();
//     update_credentials(request_state, json).await;
//     let login_history = helper
//         .get_login_history(&stored_credentials.id)
//         .await
//         .unwrap();
//     helper.delete_credentials_by_name(&name).await;
//     assert_eq!(login_history.len(), 0);
// }
//
//     #[actix_rt::test]
//     async fn deletes_login_history_if_previous_update_failures_are_expired() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let stored_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         let expired_timestamp =
//             SystemTime::now().sub(Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS + 1));
//         helper
//             .set_login_history(&model::FailedLogin {
//                 user_id: stored_credentials.id,
//                 updated_at: expired_timestamp,
//                 created_at: expired_timestamp,
//                 attempts: ALLOWED_FAILED_LOGIN_ATTEMPTS,
//             })
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         update_credentials(request_state, json).await;
//         let login_history = helper
//             .get_login_history(&stored_credentials.id)
//             .await
//             .unwrap();
//         helper.delete_credentials_by_name(&name).await;
//         assert_eq!(login_history.len(), 0);
//     }
//
//     #[actix_rt::test]
//     async fn does_not_suspend_user_if_previous_update_failures_are_expired() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let stored_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         let expired_timestamp =
//             SystemTime::now().sub(Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS + 1));
//         helper
//             .set_login_history(&model::FailedLogin {
//                 user_id: stored_credentials.id,
//                 updated_at: expired_timestamp,
//                 created_at: expired_timestamp,
//                 attempts: ALLOWED_FAILED_LOGIN_ATTEMPTS,
//             })
//             .await;
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         update_credentials(request_state, json).await;
//         let user_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         helper.delete_credentials_by_name(&name).await;
//         assert_eq!(user_credentials.locked_at, None);
//     }
//
//     #[actix_rt::test]
//     async fn creates_a_log_of_failed_login_attempts() {
//         let helper = test_helper::Helper::new().await.unwrap();
//         let db = model::DatabaseConnection::new(TEST_DATABASE_CONFIG)
//             .await
//             .unwrap();
//         let request_state = web::Data::new(model::ServiceState::new(db).await.unwrap());
//         let (name, email, password) = test_helper::fake_credentials();
//         let hashed_password = controller::password::hash_password(&password).unwrap();
//         let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
//         let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
//         let data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
//         helper
//             .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
//             .await;
//         let stored_credentials = helper
//             .get_credentials_by_name(&name)
//             .await
//             .unwrap()
//             .unwrap();
//         let (req, mut payload) = test::TestRequest::put().set_json(&data).to_http_parts();
//         let json = web::Json::<model::UpdateCredentials>::from_request(&req, &mut payload)
//             .await
//             .unwrap();
//         let resp = update_credentials(request_state, json).await;
//         let login_history = helper
//             .get_login_history(&stored_credentials.id)
//             .await
//             .unwrap();
//         helper.delete_credentials_by_name(&name).await;
//         assert_eq!(login_history.len(), 1);
//     }
// }
