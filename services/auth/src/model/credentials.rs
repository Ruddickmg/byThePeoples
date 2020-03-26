pub type CredentialId = i32;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub id: CredentialId,
    pub email: String,
    pub name: String,
    pub hash: String,
}

impl Credentials {
    pub fn from(rows: database::Results) -> Option<Credentials> {
        match rows.first() {
            Some(result) => Some(Credentials {
                id: result.get(0),
                email: result.get(1),
                name: result.get(2),
                hash: result.get(3),
            }),
            None => None,
        }
    }
}

use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub email: String,
    pub name: String,
    pub password: String,
}

impl From<web::Json<Request>> for Request {
    fn from(json: web::Json<Request>) -> Request {
        Request {
            email: String::from(&json.email),
            name: String::from(&json.name),
            password: String::from(&json.password),
        }
    }
}
