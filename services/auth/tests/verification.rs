extern crate btp_auth_server;
mod helper;
use actix_rt;
use actix_web::{http, test, App};
use btp_auth_server::{
    configuration::{ACCOUNT_LOCK_DURATION_IN_SECONDS, ALLOWED_FAILED_LOGIN_ATTEMPTS},
    controller, model, routes,
    routes::VERIFICATION_ROUTE,
};
use std::{
    ops::Sub,
    time::{Duration, SystemTime},
};

#[actix_rt::test]
async fn authenticate_credentials_success_status() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let request_data = model::NameRequest::new(&name, &password);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
    assert_eq!(resp.status(), status_codes::OKAY);
}

#[actix_rt::test]
async fn authenticate_credentials_sets_auth_header() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let request_data = model::NameRequest::new(&name, &password);
    db.add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
        .await;
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
    assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
}

#[actix_rt::test]
async fn errors_with_unauthorized_if_no_record_exists() {
    let data = helper::init_data().await;
    let (name, _email, password) = helper::fake_credentials();
    let request_data = model::NameRequest::new(&name, &password);
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
async fn errors_with_unauthorized_if_passwords_do_not_match() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    db.add_credentials(&model::FullRequest::new(&name, &hashed_password, &email))
        .await;
    let request_data = model::NameRequest::new(&name, "invalid password");
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
async fn returns_unauthorized_if_a_user_has_been_suspended() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    db.add_credentials(&model::FullRequest::new(&name, &hashed_password, &email))
        .await;
    let request_data = model::NameRequest::new(&name, "invalid password");
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.suspend_user(&stored_credentials.id).await;
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
async fn suspends_a_user_if_they_have_exceeded_the_allowed_failed_login_attempts() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    db.add_credentials(&model::FullRequest::new(&name, &hashed_password, &email))
        .await;
    let request_data = model::NameRequest::new(&name, "invalid password");
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
        .await;
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
    db.add_credentials(&model::FullRequest::new(&name, &hashed_password, &email))
        .await;
    let request_data = model::NameRequest::new(&name, "invalid password");
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
        .await;
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
async fn deletes_login_history_if_previous_login_failures_are_expired() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    db.add_credentials(&model::FullRequest::new(&name, &hashed_password, &email))
        .await;
    let request_data = model::NameRequest::new(&name, "invalid password");
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
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
async fn does_not_suspend_user_if_previous_login_failures_are_expired() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    db.add_credentials(&model::FullRequest::new(&name, &hashed_password, &email))
        .await;
    let request_data = model::NameRequest::new(&name, "invalid password");
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
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
    db.add_credentials(&model::FullRequest::new(&name, &hashed_password, &email))
        .await;
    let request_data = model::NameRequest::new(&name, "invalid password");
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    let req = test::TestRequest::post()
        .uri(VERIFICATION_ROUTE)
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
