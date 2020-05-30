use actix_web::{web, HttpResponse};
use crate::{
    repository,
    controller::password_reset,
    model,
};
use crate::controller::password_reset::ResetResult;

pub async fn reset_password<L, C, R>(
    state: web::Data<model::ServiceState<L, C, R>>,
    json: web::Json<model::ResetConfirmation>,
) -> HttpResponse
    where
        L: repository::LoginHistory,
        C: repository::Credentials,
        R: repository::PasswordResetRequest
{
    let request = model::ResetConfirmation::from(json);
    password_reset::reset_password(&state.reset_request, &state.credentials, &request)
        .await.map_or(HttpResponse::InternalServerError(), || HttpResponse::Ok())
        .finish()
}
