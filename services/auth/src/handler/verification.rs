use crate::{
    controller::{authorization, jwt},
    model,
};
use actix_web::{web, HttpResponse};

pub async fn authenticate_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::CredentialsRequest>,
) -> HttpResponse {
    let user_credentials = match model::AuthRequest::from(json) {
        model::AuthRequest::Full(full) => model::NameRequest::from(full),
        model::AuthRequest::Name(name_request) => name_request,
        _ => return HttpResponse::BadRequest().finish(),
    };
    if let Ok(stored_credentials) = authorization::authorize(&user_credentials, &state.db).await {
        match stored_credentials {
            authorization::Results::Valid(credentials) => {
                match jwt::set_token(HttpResponse::Ok(), credentials) {
                    Ok(authenticated_response) => authenticated_response,
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            authorization::Results::Invalid => HttpResponse::Unauthorized().finish(),
            authorization::Results::None => HttpResponse::NotFound().finish(),
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;
    use crate::{controller, model, utilities::test as test_helper};
    use actix_web::{http, test, FromRequest};

    #[actix_rt::test]
    async fn authenticate_credentials_success_status() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::NameRequest::new(&name, &password);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialsRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::OKAY);
    }

    #[actix_rt::test]
    async fn authenticate_credentials_sets_auth_header() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::NameRequest::new(&name, &password);
        helper
            .add_credentials(&model::FullRequest::new(&name, &email, &hashed_password))
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialsRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn errors_with_not_found_if_no_record_exists() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, _email, password) = test_helper::fake_credentials();
        let request_data = model::NameRequest::new(&name, &password);
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialsRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn errors_with_unauthorized_if_passwords_do_not_match() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let data = model::FullRequest::new(&name, &hashed_password, &email);
        helper.add_credentials(&data).await;
        let request_data = model::NameRequest::new(&name, "invalid password");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialsRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn errors_with_bad_request_if_name_is_missing_from_the_request() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (_name, _email, password) = test_helper::fake_credentials();
        let request_data = model::CredentialsRequest::new(&None, &None, &Some(password));
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialsRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn errors_with_bad_request_if_password_is_missing_from_the_request() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (_name, _email, password) = test_helper::fake_credentials();
        let request_data = model::CredentialsRequest::new(&None, &None, &Some(password));
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialsRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::BAD_REQUEST);
    }
}
