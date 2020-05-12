extern crate btp_auth_server;
mod helper;
use actix_rt;
use actix_web::{test, App};
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
async fn returns_unauthorized_if_no_record_exists() {
    let data = helper::init_data().await;
    let (_name, email, password) = helper::fake_credentials();
    let request_data = model::EmailRequest::new(&email, &password);
    let req = test::TestRequest::delete()
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
async fn returns_unauthorized_if_password_is_invalid() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let initial_data = model::FullRequest::new(&name, &email, &password);
    db.add_credentials(&initial_data).await;
    let request_data = model::EmailRequest::new(&email, "invalid password");
    let req = test::TestRequest::delete()
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
async fn returns_an_accepted_response_if_deletion_was_successful() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let request_data = model::FullRequest::new(&name, &email, &hashed_password);
    db.add_credentials(&request_data).await;
    let request_data = model::EmailRequest::new(&email, &password);
    let req = test::TestRequest::delete()
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
    assert_eq!(resp.status(), status_codes::ACCEPTED);
}

#[actix_rt::test]
async fn sets_deleted_at_timestamp_on_deleted_record() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let db_data = model::FullRequest::new(&name, &email, &hashed_password);
    db.add_credentials(&db_data).await;
    let request_data = model::EmailRequest::new(&email, &password);
    let req = test::TestRequest::delete()
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
    assert_ne!(stored_credentials.deleted_at, None);
}

#[actix_rt::test]
async fn returns_unauthorized_if_a_user_has_been_suspended() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let db_data = model::FullRequest::new(&name, &email, &hashed_password);
    db.add_credentials(&db_data).await;
    let request_data = model::EmailRequest::new(&email, &password);
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.suspend_user(&stored_credentials.id).await;
    let req = test::TestRequest::delete()
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
async fn suspends_a_user_if_they_have_exceeded_the_allowed_failed_deletion_attempts() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let db_data = model::FullRequest::new(&name, &email, &hashed_password);
    db.add_credentials(&db_data).await;
    let request_data = model::EmailRequest::new(&email, "Bad Password");
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
        .await;
    let req = test::TestRequest::delete()
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
    let db_data = model::FullRequest::new(&name, &email, &hashed_password);
    db.add_credentials(&db_data).await;
    let request_data = model::EmailRequest::new(&email, "Bad Password");
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
        .await;
    let req = test::TestRequest::delete()
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
async fn deletes_login_history_if_previous_deletion_failures_are_expired() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let db_data = model::FullRequest::new(&name, &email, &hashed_password);
    db.add_credentials(&db_data).await;
    let request_data = model::EmailRequest::new(&email, "Bad Password");
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
        .await;
    let req = test::TestRequest::delete()
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
async fn does_not_suspend_user_if_previous_deletion_failures_are_expired() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let db_data = model::FullRequest::new(&name, &email, &hashed_password);
    db.add_credentials(&db_data).await;
    let request_data = model::EmailRequest::new(&email, "Bad Password");
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
    let req = test::TestRequest::delete()
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
async fn creates_a_log_of_failed_deletion_attempts() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let hashed_password = controller::password::hash_password(&password).unwrap();
    let db_data = model::FullRequest::new(&name, &email, &hashed_password);
    db.add_credentials(&db_data).await;
    let request_data = model::EmailRequest::new(&email, "Bad Password");
    let stored_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    let req = test::TestRequest::delete()
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
