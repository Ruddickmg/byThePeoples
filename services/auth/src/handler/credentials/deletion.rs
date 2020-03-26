use crate::{controller::credentials, model};
use actix_web::{web, HttpResponse};

pub async fn delete_credentials(
    state: web::Data<model::ServiceState>,
    json: web::Json<model::CredentialRequest>,
) -> HttpResponse {
    let db = state.db.lock().unwrap();
    let user_credentials = model::CredentialRequest::from(json);
    if let Ok(deletion) = credentials::delete(db, user_credentials).await {
        match deletion {
            credentials::DeleteResults::Success => HttpResponse::Accepted(),
            credentials::DeleteResults::NotFound => HttpResponse::NotFound(),
            credentials::DeleteResults::Unauthorized => HttpResponse::Unauthorized(),
        }
        .finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
