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
    match credentials::create(&state.credentials, &user_credentials).await {
        Ok(result) => match result {
            credentials::SaveResults::Conflict => HttpResponse::Conflict().finish(),
            credentials::SaveResults::WeakPassword(problems) => serde_json::to_string(&problems)
                .map_or(HttpResponse::InternalServerError().finish(), |json| {
                    HttpResponse::Forbidden().json2(&json)
                }),
            credentials::SaveResults::Success(stored_credentials) => {
                jwt::set_token(HttpResponse::Created(), stored_credentials)
                    .unwrap_or(HttpResponse::InternalServerError().finish())
            }
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
