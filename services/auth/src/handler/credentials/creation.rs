use crate::{
    controller::{credentials, jwt, password},
    model, repository, Error,
};
use actix_web::{web, HttpResponse};
use std::sync::MutexGuard;

pub async fn save_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::CredentialRequest>,
) -> HttpResponse {
    let user_credentials = model::CredentialRequest::from(json);
    let db = state.db.lock().unwrap();
    if let Ok(result) = credentials::save(db, user_credentials).await {
        match result {
            credentials::SaveResults::Conflict => HttpResponse::Conflict().finish(),
            credentials::SaveResults::WeakPassword => HttpResponse::Forbidden().finish(),
            credentials::SaveResults::Saved(stored_credentials) => {
                if let Ok(response) = jwt::set_token(HttpResponse::Created(), stored_credentials) {
                    response
                } else {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[cfg(test)]
mod credentials_handler_tests {
    use super::*;
    use super::*;
    use crate::{controller::password, model, utilities::test as test_helper};
    use actix_web::{http, test, FromRequest};

    #[actix_rt::test]
    async fn save_credentials_success_status() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from(&password),
            email: String::from(&email),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::CREATED);
    }

    #[actix_rt::test]
    async fn save_credentials_creates_record() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from(&password),
            email: String::from(&email),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        let saved_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(&saved_credentials.name, &name);
        assert_eq!(&saved_credentials.email, &email);
    }

    #[actix_rt::test]
    async fn save_credentials_sets_auth_token() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from(&password),
            email: String::from(&email),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_conflict_if_email_exists() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let mut request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from(&password),
            email: String::from(&email),
        };
        helper.add_credentials(request_data.clone()).await;
        request_data.name = String::from("different name");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::CONFLICT);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_token_if_email_exists() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let mut request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from(&password),
            email: String::from(&email),
        };
        helper.add_credentials(request_data.clone()).await;
        request_data.name = String::from("different name");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_conflict_if_name_exists() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let mut request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from(&password),
            email: String::from(&email),
        };
        helper.add_credentials(request_data.clone()).await;
        request_data.email = String::from("different email");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::CONFLICT);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_token_if_name_exists() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let mut request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from(&password),
            email: String::from(&email),
        };
        helper.add_credentials(request_data.clone()).await;
        request_data.email = String::from("different email");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_forbidden_if_password_is_too_weak() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, ..) = helper.fake_credentials();
        let request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from("password"),
            email: String::from(&email),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::FORBIDDEN);
    }

    #[actix_rt::test]
    async fn does_not_set_auth_token_if_password_is_too_weak() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, ..) = helper.fake_credentials();
        let request_data = model::CredentialRequest {
            name: String::from(&name),
            password: String::from("password"),
            email: String::from(&email),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
    }
}
