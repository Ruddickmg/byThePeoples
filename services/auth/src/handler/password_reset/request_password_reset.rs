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
        .map_or(
            HttpResponse::InternalServerError().finish(),
            | record | HttpResponse::Accepted().json2(&record))
}

#[cfg(test)]
mod tests {
    use actix_rt;
    use super::*;
    use crate::{utilities::test::fake, error::Error};

    #[actix_rt::test]
    async fn returns_accepted_when_a_record_cannot_be_created() {
        let mut state = fake::service_state();
        let reset_request = fake::reset_request();
        let reset_record = fake::password_reset_request();
        state.reset_request.generate.returns(Some(reset_record.clone()));
        let result = request_password_reset(web::Data::new(state), web::Json(reset_request))
            .await;
        assert_eq!(result.status(), status_codes::ACCEPTED);
    }

    #[actix_rt::test]
    async fn returns_accepted_when_a_record_is_created() {
        let mut state = fake::service_state();
        let reset_request = fake::reset_request();
        state.reset_request.generate.returns(None);
        let result = request_password_reset(web::Data::new(state), web::Json(reset_request))
            .await;
        assert_eq!(result.status(), status_codes::ACCEPTED);
    }

    #[actix_rt::test]
    async fn returns_internal_server_error_when_an_unexpected_error_occurs() {
        let mut state = fake::service_state();
        let reset_request = fake::reset_request();
        state.reset_request.generate.throws_error(Error::InternalServerError(String::from("Somethings amiss")));
        let result = request_password_reset(web::Data::new(state), web::Json(reset_request))
            .await;
        assert_eq!(result.status(), status_codes::INTERNAL_SERVER_ERROR);
    }
}