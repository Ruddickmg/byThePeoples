use crate::{
    controller::{authorization, jwt},
    model, repository,
};
use actix_web::{web, HttpResponse};

pub async fn authenticate_credentials<
    T: model::Database,
    L: repository::LoginHistory<T>,
    C: repository::Credentials<T>,
>(
    state: web::Data<model::ServiceState<T, L, C>>,
    json: web::Json<model::NameRequest>,
) -> HttpResponse {
    let user_credentials = model::NameRequest::from(json);
    match authorization::authorize(&user_credentials, &state.credentials, &state.login_history)
        .await
    {
        Ok(stored_credentials) => match stored_credentials {
            authorization::Results::Valid(credentials) => {
                jwt::set_token(HttpResponse::Ok(), credentials)
                    .unwrap_or(HttpResponse::InternalServerError().finish())
            }
            _ => HttpResponse::Unauthorized().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
