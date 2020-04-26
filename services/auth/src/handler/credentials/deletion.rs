use crate::{constants::SUSPENDED_ACCOUNT_MESSAGE, controller::credentials, model};
use actix_web::{web, HttpResponse};

pub async fn delete_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::EmailRequest>,
) -> HttpResponse {
    let user_credentials = model::EmailRequest::from(json);
    match credentials::delete(&state.db, user_credentials).await {
        Ok(deletion) => match deletion {
            credentials::DeleteResults::Success => HttpResponse::Accepted().finish(),
            _ => HttpResponse::Unauthorized().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[cfg(test)]
mod credential_deletion_test {
    use super::*;
    use crate::configuration::ALLOWED_FAILED_LOGIN_ATTEMPTS;
    use crate::{controller, model, utilities::test as test_helper};
    use actix_rt;
    use actix_web::{test, web, FromRequest};

    #[actix_rt::test]
    async fn returns_unauthorized_if_no_record_exists() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (_name, email, password) = test_helper::fake_credentials();
        let data = model::EmailRequest::new(&email, &password);
        let request = test::TestRequest::delete();
        let (req, mut payload) = request.set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn returns_unauthorized_if_password_is_invalid() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let initial_data = model::FullRequest::new(&name, &email, &password);
        helper.add_credentials(&initial_data).await;
        let data = model::EmailRequest::new(&email, "invalid password");
        let request = test::TestRequest::delete();
        let (req, mut payload) = request.set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
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
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::FullRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        let request = test::TestRequest::delete();
        let data = model::EmailRequest::new(&email, &password);
        let (req, mut payload) = request.set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
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
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::FullRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        let data = model::EmailRequest::new(&email, &password);
        let (req, mut payload) = test::TestRequest::delete().set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
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

    // TODO test suspension

    #[actix_rt::test]
    async fn returns_unauthorized_if_a_user_has_been_suspended() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::FullRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        let data = model::EmailRequest::new(&email, &password);
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper.suspend_user(&stored_credentials.id).await;
        let (req, mut payload) = test::TestRequest::delete().set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn suspends_a_user_if_they_have_exceeded_the_allowed_failed_login_attempts() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::FullRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        let data = model::EmailRequest::new(&email, &password);
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper
            .set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
            .await;
        let (req, mut payload) = test::TestRequest::delete().set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn deletes_the_login_history_once_a_user_has_been_suspended() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::FullRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        let data = model::EmailRequest::new(&email, &password);
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper
            .set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS + 1))
            .await;
        let (req, mut payload) = test::TestRequest::delete().set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(resp.status(), status_codes::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn deletes_the_failed_login_history_if_a_user_successfully_logs_in() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::FullRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        let data = model::EmailRequest::new(&email, &password);
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        helper
            .set_login_attempts(&stored_credentials.id, &(ALLOWED_FAILED_LOGIN_ATTEMPTS - 1))
            .await;
        let (req, mut payload) = test::TestRequest::delete().set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        let login_history = helper
            .get_login_history(&stored_credentials.id)
            .await
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(login_history.len(), 0);
    }

    #[actix_rt::test]
    async fn creates_a_log_of_failed_login_attempts() {
        let helper = test_helper::Helper::new().await.unwrap();
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
        let (name, email, password) = test_helper::fake_credentials();
        let hashed_password = controller::password::hash_password(&password).unwrap();
        let request_data = model::FullRequest::new(&name, &email, &hashed_password);
        helper.add_credentials(&request_data).await;
        let data = model::EmailRequest::new(&email, "incorrect password");
        let stored_credentials = helper
            .get_credentials_by_name(&name)
            .await
            .unwrap()
            .unwrap();
        let (req, mut payload) = test::TestRequest::delete().set_json(&data).to_http_parts();
        let json = web::Json::<model::EmailRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = delete_credentials(request_state, json).await;
        let login_history = helper
            .get_login_history(&stored_credentials.id)
            .await
            .unwrap();
        helper.delete_credentials_by_name(&name).await;
        assert_eq!(login_history.len(), 1);
    }
}
