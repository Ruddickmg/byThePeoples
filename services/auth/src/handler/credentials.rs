use crate::{
    authentication::{jwt, password},
    model, repository, Error,
};
use actix_web::{web, HttpResponse};
use std::sync::MutexGuard;

async fn handle_credential_saving(
    db: MutexGuard<'_, model::Database>,
    model::CredentialRequest {
        name,
        email,
        password,
    }: model::CredentialRequest,
) -> Result<Option<model::Credentials>, Error> {
    let client = db.client().await?;
    let mut credentials = repository::Credentials::new(client);
    let hash = password::hash_password(&password)?;
    let encrypted_credentials = model::CredentialRequest {
        name: String::from(&name),
        email: String::from(&email),
        password: String::from(&hash),
    };
    if let repository::credentials::Status::None =
        credentials.get_status(&encrypted_credentials).await?
    {
        credentials.save_credentials(&encrypted_credentials).await?;
        return Ok(credentials.by_name(&name).await?);
    }
    Ok(None)
}

pub async fn save_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::CredentialRequest>,
) -> HttpResponse {
    let credential = model::CredentialRequest::from(json);
    if password::PasswordStrength::Weak < password::strength(&credential.password) {
        let db = state.db.lock().unwrap();
        if let Ok(result) = handle_credential_saving(db, credential).await {
            match result {
                Some(stored_credentials) => {
                    if let Ok(response) =
                        jwt::set_auth_header(HttpResponse::Created(), stored_credentials)
                    {
                        return response;
                    }
                }
                None => return HttpResponse::Conflict().finish(),
            }
        }
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Forbidden().finish()
}

#[cfg(test)]
mod credentials_handler_tests {
    use super::*;
    use super::*;
    use crate::{authentication::password, model, utilities::test as test_helper};
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
        let mut request_data = model::CredentialRequest {
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
