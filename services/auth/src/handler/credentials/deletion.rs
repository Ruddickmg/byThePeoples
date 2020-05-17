use crate::{controller::credentials, model, repository};
use actix_web::{web, HttpResponse};

pub async fn delete_credentials<
    T: model::Database,
    L: repository::LoginHistory<T>,
    C: repository::Credentials<T>,
>(
    state: web::Data<model::ServiceState<T, L, C>>,
    json: web::Json<model::EmailRequest>,
) -> HttpResponse {
    let user_credentials = model::EmailRequest::from(json);
    match credentials::delete(&state.credentials, &state.login_history, &user_credentials).await {
        Ok(deletion) => match deletion {
            credentials::DeleteResults::Success => HttpResponse::Accepted(),
            _ => HttpResponse::Unauthorized(),
        },
        Err(_) => HttpResponse::InternalServerError(),
    }
    .finish()
}
