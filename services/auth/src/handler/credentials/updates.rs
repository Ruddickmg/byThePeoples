use crate::{
    controller::{credentials, jwt},
    model, repository,
};
use actix_web::{web, HttpResponse};

pub async fn update_credentials<
    T: model::Database,
    L: repository::LoginHistory<T>,
    C: repository::Credentials<T>,
>(
    state: web::Data<model::ServiceState<T, L, C>>,
    json: web::Json<model::UpdateCredentials>,
) -> HttpResponse {
    let updated_credentials = model::UpdateCredentials::from(json);
    let model::UpdateCredentials {
        auth,
        credentials: updates,
    } = updated_credentials;
    if let Ok(status) =
        credentials::update(&state.credentials, &state.login_history, &auth, &updates).await
    {
        match status {
            credentials::UpdateResults::Success(credentials) => {
                jwt::set_token(HttpResponse::Ok(), credentials)
                    .unwrap_or(HttpResponse::InternalServerError().finish())
            }
            _ => HttpResponse::Unauthorized().finish(),
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
