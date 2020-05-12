extern crate btp_auth_server;
pub mod helper;
use actix_rt;
use actix_web::{http, test, App};
use btp_auth_server::{model, routes, routes::CREDENTIALS_ROUTE};

const WEAK_PASSWORD: &str = "password";

#[actix_rt::test]
async fn save_credentials_success_status() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let request_data = model::FullRequest::new(&name, &email, &password);
    let req = test::TestRequest::post()
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
    assert_eq!(resp.status(), status_codes::CREATED);
}

#[actix_rt::test]
async fn save_credentials_creates_record() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let request_data = model::FullRequest::new(&name, &email, &password);
    let req = test::TestRequest::post()
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
    let saved_credentials = db.get_credentials_by_name(&name).await.unwrap().unwrap();
    db.delete_credentials_by_name(&name).await;
    assert_eq!(&saved_credentials.name, &name);
    assert_eq!(&saved_credentials.email, &email);
}

#[actix_rt::test]
async fn save_credentials_sets_auth_token() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let request_data = model::FullRequest::new(&name, &email, &password);
    let req = test::TestRequest::post()
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
    assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
}

#[actix_rt::test]
async fn returns_conflict_if_email_exists() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let mut request_data = model::FullRequest::new(&name, &email, &password);
    db.add_credentials(&request_data).await;
    request_data.name = String::from("different name");
    let req = test::TestRequest::post()
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
    assert_eq!(resp.status(), status_codes::CONFLICT);
}

#[actix_rt::test]
async fn does_not_set_auth_token_if_email_exists() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let mut request_data = model::FullRequest::new(&name, &email, &password);
    db.add_credentials(&request_data).await;
    request_data.name = String::from("different name");
    let req = test::TestRequest::post()
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
async fn returns_conflict_if_name_exists() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let mut request_data = model::FullRequest::new(&name, &email, &password);
    db.add_credentials(&request_data).await;
    request_data.email = String::from("different email");
    let req = test::TestRequest::post()
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
    assert_eq!(resp.status(), status_codes::CONFLICT);
}

#[actix_rt::test]
async fn does_not_set_auth_token_if_name_exists() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let mut request_data = model::FullRequest::new(&name, &email, &password);
    db.add_credentials(&request_data).await;
    request_data.email = String::from("different email");
    let req = test::TestRequest::post()
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
async fn returns_forbidden_if_password_is_too_weak() {
    let data = helper::init_data().await;
    let (name, email, ..) = helper::fake_credentials();
    let request_data = model::FullRequest::new(&name, &email, WEAK_PASSWORD);
    let req = test::TestRequest::post()
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
    assert_eq!(resp.status(), status_codes::FORBIDDEN);
}

#[actix_rt::test]
async fn does_not_set_auth_token_if_password_is_too_weak() {
    let data = helper::init_data().await;
    let (name, email, ..) = helper::fake_credentials();
    let request_data = model::FullRequest::new(&name, &email, WEAK_PASSWORD);
    let req = test::TestRequest::post()
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
