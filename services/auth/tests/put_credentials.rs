extern crate btp_auth_server;
mod helper;
use actix_rt;
use actix_web::{http, test, App};
use btp_auth_server::{
    configuration::{ACCOUNT_LOCK_DURATION_IN_SECONDS, ALLOWED_FAILED_LOGIN_ATTEMPTS},
    controller, model, routes,
    routes::CREDENTIALS_ROUTE,
};
use std::{
    ops::Sub,
    time::{Duration, SystemTime},
};

#[actix_rt::test]
async fn returns_okay_if_the_update_was_successful() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let (name2, email2, password2) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, &password);
    let updated_credentials = model::CredentialsRequest::new(
        &Some(name2.clone()),
        &Some(email2.clone()),
        &Some(password2.clone()),
    );
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&name2).await;
    assert_eq!(resp.status(), status_codes::OKAY);
}

#[actix_rt::test]
async fn sets_updated_auth_token_on_successful_response() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let (name2, email2, password2) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, &password);
    let updated_credentials = model::CredentialsRequest::new(
        &Some(name2.clone()),
        &Some(email2.clone()),
        &Some(password2.clone()),
    );
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&name2).await;
    assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
}

#[actix_rt::test]
async fn updates_a_users_name() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let (name2, ..) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, &password);
    let updated_credentials = model::CredentialsRequest::new(&Some(name2.clone()), &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    test::call_service(&mut server, req).await;
    let stored_credentials = db.get_credentials_by_name(&name2).await.unwrap().unwrap();
    db.delete_credentials_by_name(&name2).await;
    assert_eq!(&stored_credentials.name, &name2);
    assert_eq!(&stored_credentials.email, &email);
    assert_eq!(&stored_credentials.hash, &hashed_password);
}

#[actix_rt::test]
async fn updates_a_users_password() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let (name2, _email2, password2) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, &password);
    let updated_credentials =
        model::CredentialsRequest::new(&None, &None, &Some(password2.clone()));
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    test::call_service(&mut server, req).await;
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.delete_credentials_by_name(&name2).await;
    assert_eq!(&stored_credentials.email, &email);
    assert_eq!(&stored_credentials.name, &name);
    assert!(controller::password::authenticate(&password2, &stored_credentials.hash).unwrap());
}

#[actix_rt::test]
async fn updates_a_users_email() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let (_name2, email2, ..) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, &password);
    let updated_credentials = model::CredentialsRequest::new(&None, &Some(email2.clone()), &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    test::call_service(&mut server, req).await;
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.delete_credentials_by_name(&name).await;
    assert_eq!(&stored_credentials.email, &email2);
    assert_eq!(&stored_credentials.hash, &hashed_password);
    assert_eq!(&stored_credentials.name, &name);
}

#[actix_rt::test]
async fn returns_unauthorized_if_auth_credentials_are_invalid() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&name).await;
    assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
}

#[actix_rt::test]
async fn does_not_set_auth_token_if_unauthorized() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, "Invalid Password");
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&name).await;
    assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
}

#[actix_rt::test]
async fn returns_unauthorized_if_no_associated_record_exists() {
    let data = helper::init_data().await;
    let (_name, email, password) = helper::fake_credentials();
    let auth_credentials = model::EmailRequest::new(&email, &password);
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    let resp = test::call_service(&mut server, req).await;
    assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
}

#[actix_rt::test]
async fn doest_not_set_auth_token_if_not_found() {
    let data = helper::init_data().await;
    let (_name, email, password) = helper::fake_credentials();
    let auth_credentials = model::EmailRequest::new(&email, &password);
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    let resp = test::call_service(&mut server, req).await;
    assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
}

#[actix_rt::test]
async fn returns_unauthorized_if_a_user_has_been_suspended() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, &password);
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.suspend_user(&stored_credentials.id).await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&name).await;
    assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
}

#[actix_rt::test]
async fn suspends_a_user_if_they_have_exceeded_the_allowed_failed_update_attempts() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, "Invalid password");
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    test::call_service(&mut server, req).await;
    let user_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.delete_credentials_by_name(&name).await;
    assert_ne!(user_credentials.locked_at, None);
}

#[actix_rt::test]
async fn deletes_the_login_history_once_a_user_has_been_suspended() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, "Invalid password");
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
        .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    test::call_service(&mut server, req).await;
    let login_history = db.get_login_history(&stored_credentials.id).await.unwrap();
    db.delete_credentials_by_name(&name).await;
    assert_eq!(login_history.len(), 0);
}

#[actix_rt::test]
async fn deletes_login_history_if_previous_update_failures_are_expired() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, "Invalid password");
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    let expired_timestamp =
        SystemTime::now().sub(Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS + 1));
    db.set_login_history(&model::FailedLogin {
        user_id: stored_credentials.id,
        updated_at: expired_timestamp,
        created_at: expired_timestamp,
        attempts: ALLOWED_FAILED_LOGIN_ATTEMPTS,
    })
    .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    test::call_service(&mut server, req).await;
    let login_history = db.get_login_history(&stored_credentials.id).await.unwrap();
    db.delete_credentials_by_name(&name).await;
    assert_eq!(login_history.len(), 0);
}

#[actix_rt::test]
async fn does_not_suspend_user_if_previous_update_failures_are_expired() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, "Invalid password");
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    let expired_timestamp =
        SystemTime::now().sub(Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS + 1));
    db.set_login_history(&model::FailedLogin {
        user_id: stored_credentials.id,
        updated_at: expired_timestamp,
        created_at: expired_timestamp,
        attempts: ALLOWED_FAILED_LOGIN_ATTEMPTS,
    })
    .await;
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    test::call_service(&mut server, req).await;
    let user_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.delete_credentials_by_name(&name).await;
    assert_eq!(user_credentials.locked_at, None);
}

#[actix_rt::test]
async fn creates_a_log_of_failed_login_attempts() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let auth_credentials = model::EmailRequest::new(&email, "Invalid password");
    let updated_credentials = model::CredentialsRequest::new(&None, &None, &None);
    let request_data = model::UpdateCredentials::new(&auth_credentials, &updated_credentials);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    let req = test::TestRequest::put()
        .uri(CREDENTIALS_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
    .await;
    test::call_service(&mut server, req).await;
    let login_history = db.get_login_history(&stored_credentials.id).await.unwrap();
    db.delete_credentials_by_name(&name).await;
    assert_eq!(login_history.len(), 1);
}
