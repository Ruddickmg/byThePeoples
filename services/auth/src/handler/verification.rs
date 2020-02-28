use crate::authentication::{jwt, password};
use crate::model::credentials;
use crate::{logging, model};
use actix_web::{http, web, HttpResponse};

fn server_error(error: String) -> HttpResponse {
    logging::log_error(error);
    HttpResponse::InternalServerError().body("Oops! Something went wrong!")
}

fn invalid_credentials(name: &str) -> HttpResponse {
    // probably count amount of invalid login
    println!("Invalid login made by {}", name);
    HttpResponse::Unauthorized().finish()
}

pub async fn authenticate_credentials(
    state: web::Data<model::ServiceState>,
    credentials: web::Json<model::AuthRequest>,
) -> HttpResponse {
    let mut db = state.db.lock().unwrap();
    let client = db.client().await.unwrap();
    let mut auth_credentials = credentials::Model::new(client);
    let user_name = &credentials.name;
    if let Ok(maybe_auth_record) = auth_credentials.by_name(&user_name).await {
        match maybe_auth_record {
            Some(auth_record) => {
                match password::authenticate(&credentials.password, &auth_record.hash) {
                    Ok(correct_password) => {
                        if correct_password {
                            match jwt::generate_token(auth_record) {
                                Ok(token) => HttpResponse::Ok()
                                    .header(
                                        http::header::AUTHORIZATION,
                                        format!("Bearer {}", token),
                                    )
                                    .finish(),
                                Err(error) => server_error(error),
                            }
                        } else {
                            invalid_credentials(&user_name)
                        }
                    }
                    Err(error) => server_error(error),
                }
            }
            None => HttpResponse::NotFound().finish(),
        }
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;
    use actix_web::{http, test, FromRequest};

    #[actix_rt::test]
    async fn authenticate_credentials_success_status() {
        let db: model::Database = model::initialize().await.unwrap();
        let request_state = web::Data::new(model::ServiceState { db });
        let request_data = model::AuthRequest {
            name: String::from("hello"),
            password: String::from("world"),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::AuthRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn authenticate_credentials_header() {
        let db: model::Database = model::initialize().await.unwrap();
        let request_state = web::Data::new(model::ServiceState { db });
        let request_data = model::AuthRequest {
            name: String::from("hello"),
            password: String::from("world"),
        };
        let (req, mut payload) = test::TestRequest::post()
            .set_json(&request_data)
            .to_http_parts();
        let json = web::Json::<model::AuthRequest>::from_request(&req, &mut payload)
            .await
            .unwrap();
        let resp = authenticate_credentials(request_state, json).await;
        assert!(resp.headers().contains_key(http::header::AUTHORIZATION));
    }
}
