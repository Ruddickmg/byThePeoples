use crate::{authentication::authorization, model, Error};
use actix_web::{http, web, HttpResponse};

pub async fn authenticate_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::AuthRequest>,
) -> HttpResponse {
    let user_credentials = model::AuthRequest::from(json);
    let db = state.db.lock().unwrap();
    match authorization::authorize(user_credentials, db).await {
        Ok(potential_token) => match potential_token {
            Some(auth_token) => HttpResponse::Ok()
                .header(
                    http::header::AUTHORIZATION,
                    format!("Bearer {}", auth_token),
                )
                .finish(),
            None => HttpResponse::NotFound().finish(),
        },
        Err(error) => match error {
            Error::Unauthorized => HttpResponse::Unauthorized(),
            _ => HttpResponse::InternalServerError(),
        }
        .finish(),
    }
}

#[cfg(test)]
mod auth_tests {
    use super::*;
    use actix_web::{http, test, FromRequest};

    #[actix_rt::test]
    async fn authenticate_credentials_success_status() {
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
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
        let request_state = web::Data::new(model::ServiceState::new().await.unwrap());
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
