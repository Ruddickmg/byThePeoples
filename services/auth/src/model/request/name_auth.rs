use crate::model;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NameRequest {
    pub name: String,
    pub password: String,
}

impl NameRequest {
    pub fn new(name: &str, password: &str) -> NameRequest {
        NameRequest {
            name: String::from(name),
            password: String::from(password),
        }
    }
}

impl From<model::FullRequest> for NameRequest {
    fn from(request: model::FullRequest) -> NameRequest {
        NameRequest::new(&request.name, &request.password)
    }
}

impl From<web::Json<NameRequest>> for NameRequest {
    fn from(json: web::Json<NameRequest>) -> NameRequest {
        NameRequest {
            name: String::from(&json.name),
            password: String::from(&json.password),
        }
    }
}
