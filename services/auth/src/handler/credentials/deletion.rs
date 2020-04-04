use crate::{controller::credentials, model};
use actix_web::{web, HttpResponse};

pub async fn delete_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::CredentialRequest>,
) -> HttpResponse {
    let user_credentials = model::CredentialRequest::from(json);
    match credentials::delete(&state.db, user_credentials).await {
        Ok(deletion) => match deletion {
            credentials::DeleteResults::Success => HttpResponse::Accepted(),
            credentials::DeleteResults::NotFound => HttpResponse::NotFound(),
            credentials::DeleteResults::Unauthorized => HttpResponse::Unauthorized(),
        }
        .finish(),
        Err(e) => {
            println!("{:#?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod credential_deletion_test {
    use super::*;
    use crate::controller::password::hash_password;
    use crate::{controller, model, utilities::test as test_helper};
    use actix_rt;
    use actix_web::{test, web, FromRequest};

    #[actix_rt::test]
    async fn returns_not_found_if_no_record_exists() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let request_data = model::CredentialRequest::new(&name, &email, &password);
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn returns_unauthorized_if_password_is_invalid() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let data = model::CredentialRequest::new(&name, &email, &password);
        helper.add_credentials(&data).await;
        let mut request_data = data.clone();
        request_data.password = String::from("invalid password");
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn returns_an_accepted_response_if_deletion_was_successful() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let mut request_data = model::CredentialRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        request_data.password = password;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::ACCEPTED);
    }

    #[actix_rt::test]
    async fn sets_deleted_at_timestamp_on_deleted_record() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = helper.fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let mut request_data = model::CredentialRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        request_data.password = password;
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::CredentialRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        delete_credentials(request_state, json).await;
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_ne!(stored_credentials.deleted_at, None);
    }
}
