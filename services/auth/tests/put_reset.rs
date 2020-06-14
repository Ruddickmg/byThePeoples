extern crate btp_auth_server;
pub mod helper;
use actix_rt;
use actix_web::{test, App};
use btp_auth_server::{
    routes::PASSWORD_RESET_ROUTE,
    utilities::hash,
    routes,
    model,
};
use std::time::{SystemTime, Duration};
use std::ops::Sub;
use btp_auth_server::configuration::PASSWORD_RESET_TIME_PERIOD;

#[actix_rt::test]
async fn returns_accepted_when_password_reset_is_successful() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let id = hash::token();
    let reset_token = hash::token();
    let hashed_token = hash::generate(&reset_token).unwrap();
    let request_data = model::ResetConfirmation::new(&id, &reset_token, &password);
    let credentials = model::FullRequest::new(&name, &email, &password);
    let req = test::TestRequest::put()
        .uri(PASSWORD_RESET_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
        .await;
    db.add_credentials(&credentials).await;
    let user_id = db.get_credentials_by_name(&name).await.unwrap().unwrap().id;
    let password_reset_request = model::PasswordResetRequest {
        id: id.clone(),
        user_id,
        reset_token: hashed_token.clone(),
        name: name.clone(),
        email: email.clone(),
        created_at: SystemTime::now(),
    };
    let _ = db.add_reset_request(&password_reset_request).await.unwrap();
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&credentials.name).await;
    assert_eq!(resp.status(), status_codes::ACCEPTED);
}

#[actix_rt::test]
async fn returns_accepted_when_no_matching_request_is_found() {
    let data = helper::init_data().await;
    let (_name, _email, password) = helper::fake_credentials();
    let id = hash::token();
    let reset_token = hash::token();
    let request_data = model::ResetConfirmation::new(&id, &reset_token, &password);
    let req = test::TestRequest::put()
        .uri(PASSWORD_RESET_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
        .await;
    let resp = test::call_service(&mut server, req).await;
    assert_eq!(resp.status(), status_codes::ACCEPTED);
}

#[actix_rt::test]
async fn returns_accepted_when_an_invalid_token_is_received() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let id = hash::token();
    let invalid_token = hash::token();
    let reset_token = hash::token();
    let hashed_token = hash::generate(&reset_token).unwrap();
    let request_data = model::ResetConfirmation::new(&id, &invalid_token, &password);
    let credentials = model::FullRequest::new(&name, &email, &password);
    let req = test::TestRequest::put()
        .uri(PASSWORD_RESET_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
        .await;
    db.add_credentials(&credentials).await;
    let user_id = db.get_credentials_by_name(&name).await.unwrap().unwrap().id;
    let password_reset_request = model::PasswordResetRequest {
        id: id.clone(),
        user_id,
        reset_token: hashed_token.clone(),
        name: name.clone(),
        email: email.clone(),
        created_at: SystemTime::now(),
    };
    let _ = db.add_reset_request(&password_reset_request).await.unwrap();
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&credentials.name).await;
    assert_eq!(resp.status(), status_codes::ACCEPTED);
}

#[actix_rt::test]
async fn returns_gone_if_the_reset_request_has_expired() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let id = hash::token();
    let invalid_token = hash::token();
    let reset_token = hash::token();
    let hashed_token = hash::generate(&reset_token).unwrap();
    let request_data = model::ResetConfirmation::new(&id, &invalid_token, &password);
    let credentials = model::FullRequest::new(&name, &email, &password);
    let req = test::TestRequest::put()
        .uri(PASSWORD_RESET_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
        .await;
    db.add_credentials(&credentials).await;
    let user_id = db.get_credentials_by_name(&name).await.unwrap().unwrap().id;
    let password_reset_request = model::PasswordResetRequest {
        id: id.clone(),
        user_id,
        reset_token: hashed_token.clone(),
        name: name.clone(),
        email: email.clone(),
        created_at: SystemTime::now().sub(Duration::from_secs(PASSWORD_RESET_TIME_PERIOD * 2)),
    };
    let _ = db.add_reset_request(&password_reset_request).await.unwrap();
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&credentials.name).await;
    assert_eq!(resp.status(), status_codes::GONE);
}

#[actix_rt::test]
async fn returns_forbidden_when_a_password_is_too_weak() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let id = hash::token();
    let invalid_token = hash::token();
    let reset_token = hash::token();
    let hashed_token = hash::generate(&reset_token).unwrap();
    let request_data = model::ResetConfirmation::new(&id, &invalid_token, &helper::WEAK_PASSWORD);
    let credentials = model::FullRequest::new(&name, &email, &password);
    let req = test::TestRequest::put()
        .uri(PASSWORD_RESET_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
        .await;
    db.add_credentials(&credentials).await;
    let user_id = db.get_credentials_by_name(&name).await.unwrap().unwrap().id;
    let password_reset_request = model::PasswordResetRequest {
        id: id.clone(),
        user_id,
        reset_token: hashed_token.clone(),
        name: name.clone(),
        email: email.clone(),
        created_at: SystemTime::now(),
    };
    let _ = db.add_reset_request(&password_reset_request).await.unwrap();
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&credentials.name).await;
    assert_eq!(resp.status(), status_codes::FORBIDDEN);
}
