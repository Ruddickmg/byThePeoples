use crate::{
    controller::{credentials, jwt},
    model,
};
use actix_web::{web, HttpResponse};

pub async fn save_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::FullRequest>,
) -> HttpResponse {
    let user_credentials = model::FullRequest::from(json);
    if let Ok(result) = credentials::create(&state.db, user_credentials).await {
        match result {
            credentials::SaveResults::Conflict => HttpResponse::Conflict().finish(),
            credentials::SaveResults::WeakPassword(problems) => {
                if let Ok(json) = serde_json::to_string(&problems) {
                    HttpResponse::Forbidden().json2(&json)
                } else {
                    HttpResponse::InternalServerError().finish()
                }
            }
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
    use crate::{model, utilities::test as test_helper};
    use actix_web::{http, test, FromRequest};

    const WEAK_PASSWORD: &str = "password";

    #[actix_rt::test]
    async fn save_credentials_success_status() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let request_data = model::FullRequest::new(&name, &email, &password);
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
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
        let (name, email, password) = test_helper::fake_credentials();
        let request_data = model::FullRequest::new(&name, &email, &password);
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        save_credentials(request_state, json).await;
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
        let (name, email, password) = test_helper::fake_credentials();
        let request_data = model::FullRequest::new(&name, &email, &password);
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
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
        let (name, email, password) = test_helper::fake_credentials();
        let mut request_data = model::FullRequest::new(&name, &email, &password);
        helper.add_credentials(&request_data).await;
        request_data.name = String::from("different name");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
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
        let (name, email, password) = test_helper::fake_credentials();
        let mut request_data = model::FullRequest::new(&name, &email, &password);
        helper.add_credentials(&request_data).await;
        request_data.name = String::from("different name");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
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
        let (name, email, password) = test_helper::fake_credentials();
        let mut request_data = model::FullRequest::new(&name, &email, &password);
        helper.add_credentials(&request_data).await;
        request_data.email = String::from("different email");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
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
        let (name, email, password) = test_helper::fake_credentials();
        let mut request_data = model::FullRequest::new(&name, &email, &password);
        helper.add_credentials(&request_data).await;
        request_data.email = String::from("different email");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn returns_forbidden_if_password_is_too_weak() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, ..) = test_helper::fake_credentials();
        let request_data = model::FullRequest::new(&name, &email, WEAK_PASSWORD);
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::FORBIDDEN);
    }

    // TODO test for weak password message body

    #[actix_rt::test]
    async fn does_not_set_auth_token_if_password_is_too_weak() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, ..) = test_helper::fake_credentials();
        let request_data = model::FullRequest::new(&name, &email, WEAK_PASSWORD);
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::FullRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = save_credentials(request_state, json).await;
        assert!(!resp.headers().contains_key(http::header::AUTHORIZATION));
    }
}
