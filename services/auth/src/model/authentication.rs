use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Request {
    pub name: String,
    pub password: String,
}

impl Request {
    pub fn new(name: &str, password: &str) -> Request {
        Request {
            name: String::from(name),
            password: String::from(password),
        }
    }
}

impl From<web::Json<Request>> for Request {
    fn from(json: web::Json<Request>) -> Request {
        Request {
            name: String::from(&json.name),
            password: String::from(&json.password),
        }
    }
}
