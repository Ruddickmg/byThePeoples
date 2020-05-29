use actix_web::web;

mod credentials;
mod email_auth;
mod full_auth;
mod name_auth;
mod update;
mod password_reset;

use actix_web::web::Json;
pub use credentials::CredentialsRequest;
pub use email_auth::*;
pub use full_auth::FullRequest;
pub use name_auth::NameRequest;
pub use password_reset::*;
pub use update::*;

pub enum AuthRequest {
    Full(FullRequest),
    Name(NameRequest),
    Email(EmailRequest),
    Invalid,
}

impl From<web::Json<CredentialsRequest>> for AuthRequest {
    fn from(json: Json<CredentialsRequest>) -> Self {
        match &json.password {
            Some(password) => match &json.email {
                Some(email) => match &json.name {
                    Some(name) => AuthRequest::Full(FullRequest::new(name, email, password)),
                    None => AuthRequest::Email(EmailRequest::new(email, password)),
                },
                None => match &json.name {
                    Some(name) => AuthRequest::Name(NameRequest::new(name, password)),
                    None => AuthRequest::Invalid,
                },
            },
            None => AuthRequest::Invalid,
        }
    }
}

impl From<web::Json<NameRequest>> for AuthRequest {
    fn from(json: web::Json<NameRequest>) -> Self {
        AuthRequest::Name(NameRequest::from(json))
    }
}

impl From<web::Json<EmailRequest>> for AuthRequest {
    fn from(json: web::Json<EmailRequest>) -> Self {
        AuthRequest::Email(EmailRequest::from(json))
    }
}

impl From<web::Json<FullRequest>> for AuthRequest {
    fn from(json: web::Json<FullRequest>) -> Self {
        AuthRequest::Full(FullRequest::from(json))
    }
}
