use actix_web::{web, HttpResponse};
use crate::{
    controller::password_reset,
    repository,
    model,
};

pub async fn request_password_reset<L, C, R>(
    state: web::Data<model::ServiceState<L, C, R>>,
    json: web::Json<model::ResetRequest>,
) -> HttpResponse
    where
        L: repository::LoginHistory,
        C: repository::Credentials,
        R: repository::PasswordResetRequest
{
    let request = model::ResetRequest::from(json);
    password_reset::request_password_reset(&state.reset_request, &request.email).await
        .map_or(HttpResponse::InternalServerError().finish(), | record | {
            let response = model::ResetResponse::new(&record.id, &record.reset_token);
            HttpResponse::Accepted().json2(&response)
        })
}