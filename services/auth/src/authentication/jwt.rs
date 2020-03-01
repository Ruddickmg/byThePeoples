use super::configuration::jwt;
use crate::{model::credentials::Credentials, Error};
use jsonwebtoken;
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: u32,
    name: String,
    exp: usize,
}

pub fn generate_token(credentials: Credentials) -> Result<String, Error> {
    let Credentials { id, name, .. } = credentials;
    match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            id,
            name,
            exp: jwt::expiration(),
        },
        &EncodingKey::from_secret(&jwt::secret().as_ref()),
    ) {
        Ok(jwt) => Ok(jwt),
        Err(_) => Err(Error::from(actix_web::error::ErrorInternalServerError(
            "Failed to generate JWT",
        ))),
    }
}
