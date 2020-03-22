use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthRequest {
    pub name: String,
    pub password: String,
}

impl From<web::Json<AuthRequest>> for AuthRequest {
    fn from(json: web::Json<AuthRequest>) -> AuthRequest {
        println!("here...");
        AuthRequest {
            name: String::from(&json.name),
            password: String::from(&json.password),
        }
    }
}
