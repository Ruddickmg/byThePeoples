use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AuthRequest {
    pub name: String,
    pub password: String,
}

impl From<web::Json<AuthRequest>> for AuthRequest {
    fn from(json: web::Json<AuthRequest>) -> AuthRequest {
        AuthRequest {
            name: String::from(&json.name),
            password: String::from(&json.password),
        }
    }
}
