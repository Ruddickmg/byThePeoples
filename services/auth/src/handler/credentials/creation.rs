use crate::{
    controller::{credentials, jwt},
    model, repository,
};
use actix_web::{web, HttpResponse};

pub async fn save_credentials<
    T: model::Database,
    L: repository::LoginHistory<T>,
    C: repository::Credentials<T>,
>(
    state: web::Data<model::ServiceState<T, L, C>>,
    json: web::Json<model::FullRequest>,
) -> HttpResponse {
    let user_credentials = model::FullRequest::from(json);
    if let Ok(result) = credentials::create(&state.credentials, &user_credentials).await {
        match result {
            credentials::SaveResults::Conflict => HttpResponse::Conflict().finish(),
            credentials::SaveResults::WeakPassword(problems) => {
                if let Ok(json) = serde_json::to_string(&problems) {
                    HttpResponse::Forbidden().json2(&json)
                } else {
                    HttpResponse::InternalServerError().finish()
                }
            }
            credentials::SaveResults::Success(stored_credentials) => {
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
