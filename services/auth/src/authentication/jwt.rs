use super::configuration::jwt;
use crate::model::user::User;
use jsonwebtoken::{encode, errors, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: u32,
    name: String,
    exp: usize,
}

pub fn generate_token(User { id, name, .. }: User) -> Result<String, errors::Error> {
    encode(
        &Header::default(),
        &Claims {
            id,
            name,
            exp: jwt::expiration(),
        },
        jwt::secret().as_ref(),
    )
}
