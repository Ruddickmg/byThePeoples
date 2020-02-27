use super::configuration::jwt;
use crate::model::credentials::Credentials;
use jsonwebtoken;
use serde::{Deserialize, Serialize};

pub const TOKEN_KEY: &str = "Authorization: Bearer";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: u32,
    name: String,
    exp: usize,
}

pub fn generate_token(credentials: Credentials) -> Result<String, String> {
    let Credentials { id, name, .. } = credentials;
    let error_message = format!(
        "An error occurred while generating the jason web token for user: {}, id: {}",
        &name, &id
    );
    if let Some(valid_id) = id {
        match jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &Claims {
                id: valid_id,
                name,
                exp: jwt::expiration(),
            },
            jwt::secret().as_ref(),
        ) {
            Ok(jwt) => Ok(jwt),
            _ => Err(error_message),
        }
    } else {
        Err(error_message)
    }
}
