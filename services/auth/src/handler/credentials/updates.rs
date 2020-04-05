use crate::{
    controller::{credentials, jwt},
    model,
};
use actix_web::{web, HttpResponse};

pub async fn update(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::UpdateRequest>,
) -> HttpResponse {
    let model::UpdateRequest {
        auth,
        credentials: updates,
    }: model::UpdateRequest = model::UpdateRequest::from(json);
    if let Ok(status) = credentials::update(&state.db, &auth, &updates).await {
        match status {
            credentials::UpdateResults::Success(credentials) => {
                match jwt::set_token(HttpResponse::Ok(), credentials) {
                    Ok(authenticated_response) => authenticated_response,
                    Err(_) => HttpResponse::InternalServerError().finish(),
                }
            }
            credentials::UpdateResults::Unauthorized => HttpResponse::Unauthorized().finish(),
            credentials::UpdateResults::NotFound => HttpResponse::NotFound().finish(),
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
