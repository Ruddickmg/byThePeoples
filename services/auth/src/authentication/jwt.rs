use super::configuration::jwt;
use crate::model::credentials::Credentials;
use jsonwebtoken;
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};

pub const TOKEN_KEY: &str = "Authorization: Bearer";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: u32,
    name: String,
    exp: usize,
}

// TODO add better error handling here
pub fn generate_token(credentials: Credentials) -> Option<String> {
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
        Ok(jwt) => Some(jwt),
        Err(_) => None,
    }
}
