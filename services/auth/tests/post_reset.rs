extern crate btp_auth_server;
pub mod helper;
use actix_rt;
use actix_web::{test, App, http::header};
use btp_auth_server::{
    routes::PASSWORD_RESET_ROUTE,
    routes,
    model,
};
use std::any::Any;

#[actix_rt::test]
async fn returns_accepted_when_a_request_is_created() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let mut request_data = model::ResetRequest::new(&email);
    let credentials = model::FullRequest::new(&name, &email, &password);
    request_data.email = String::from(&credentials.email);
    let req = test::TestRequest::post()
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
    let resp = test::call_service(&mut server, req).await;
    db.delete_credentials_by_name(&credentials.name).await;
    assert_eq!(resp.status(), status_codes::ACCEPTED);
}

#[actix_rt::test]
async fn returns_a_password_reset_token_when_a_request_is_created() {
    let data = helper::init_data().await;
    let db = helper::Helper::new().await.unwrap();
    let (name, email, password) = helper::fake_credentials();
    let mut request_data = model::ResetRequest::new(&email);
    let credentials = model::FullRequest::new(&name, &email, &password);
    request_data.email = String::from(&credentials.email);
    let req = test::TestRequest::post()
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
    let body: model::ResetToken = test::read_response_json(&mut server, req).await;
    let user_id = db.get_credentials_by_name(&name).await.unwrap().unwrap().id;
    let reset_request = db.get_reset_request(&user_id).await.unwrap();
    let reset_token = model::ResetToken::new(&reset_request.id, &email);
    db.delete_credentials_by_name(&credentials.name).await;
    assert_eq!(reset_request.id, body.id);
    assert_eq!(reset_token.type_id(), body.type_id())
}

#[actix_rt::test]
async fn returns_accepted_if_no_matching_credentials_exist() {
    let data = helper::init_data().await;
    let (_name, email, _password) = helper::fake_credentials();
    let request_data = model::ResetRequest::new(&email);
    let req = test::TestRequest::post()
        .uri(PASSWORD_RESET_ROUTE)
        .header(header::CONTENT_TYPE, "application/json")
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
async fn returns_a_fake_password_reset_token_if_no_credentials_match() {
    let data = helper::init_data().await;
    let (name, email, _password) = helper::fake_credentials();
    let request_data = model::ResetRequest::new(&email);
    let reset_token = model::ResetToken::new(&name, &email);
    let req = test::TestRequest::post()
        .uri(PASSWORD_RESET_ROUTE)
        .set_json(&request_data)
        .to_request();
    let mut server = test::init_service(
        App::new()
            .app_data(data.clone())
            .configure(routes::configuration),
    )
        .await;
    let body: model::ResetToken = test::read_response_json(&mut server, req).await;
    assert_eq!(reset_token.type_id(), body.type_id());
}