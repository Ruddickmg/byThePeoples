use crate::{
    controller::{authorization, jwt},
    model, Error,
};
use actix_web::{web, HttpResponse};

pub async fn authenticate_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::AuthRequest>,
) -> HttpResponse {
    let user_credentials = model::AuthRequest::from(json);
    let db = state.db.lock().unwrap();
    match authorization::authorize(user_credentials, db).await {
        Ok(stored_credentials) => match stored_credentials {
            authorization::Results::Valid(credentials) => {
                match jwt::set_token(HttpResponse::Ok(), credentials) {
                    Ok(authenticated_response) => authenticated_response,
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            authorization::Results::Invalid => HttpResponse::Unauthorized().finish(),
            authorization::Results::None => return HttpResponse::NotFound().finish(),
        },
        Err(error) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;
    use crate::{controller::password, model, utilities::test as test_helper};
    use actix_web::{http, test, FromRequest};

    #[actix_rt::test]
    async fn authenticate_credentials_success_status() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let hashed_password = password::hash_password(&password).unwrap();
        let request_data = model::AuthRequest {
            name: String::from(&name),
            password: String::from(&password),
        };
        helper
            .add_credentials(model::CredentialRequest {
                name: String::from(&name),
                password: String::from(&hashed_password),
                email: String::from(&email),
            })
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::AuthRequest>::from_request(&req, &mut payload)
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
        let (name, email, password) = helper.fake_credentials();
        let hashed_password = password::hash_password(&password).unwrap();
        let request_data = model::AuthRequest {
            name: String::from(&name),
            password: String::from(&password),
        };
        helper
            .add_credentials(model::CredentialRequest {
                name: String::from(&name),
                password: String::from(&hashed_password),
                email: String::from(&email),
            })
            .await;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::AuthRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
    }

    #[actix_rt::test]
    async fn errors_with_not_found_if_no_record_exists() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, _email, password) = helper.fake_credentials();
        let request_data = model::AuthRequest {
            name: String::from(&name),
            password: String::from(&password),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::AuthRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn errors_with_unauthorized_if_passwords_do_not_match() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let hashed_password = password::hash_password(&password).unwrap();
        helper
            .add_credentials(model::CredentialRequest {
                name: String::from(&name),
                password: String::from(&hashed_password),
                email: String::from(&email),
            })
            .await;
        let request_data = model::AuthRequest {
            name: String::from(&name),
            password: String::from("Incorrect password"),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::AuthRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    // TODO figure out how to handle errors which occur when the arguments are incorrect (happens outside code)
    // #[actix_rt::test]
    // async fn errors_with_unprocessable_entity_if_invalid_data_is_provided() {
    //     let helper = test_helper::Helper::new().await.unwrap();
    //     let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
    //     let (req, mut payload) = test::TestRequest::post().set_json(&{}).to_http_parts();
    //     let json = web::Json::<model::AuthRequest>::from_request(&req, &mut payload)
    //         .await
    //         .unwrap();
    //     let resp = authenticate_credentials(request_state, json).await;
    //     assert_eq!(resp.status(), status_codes::UNPROCESSABLE_ENTITY);
    // }
}
