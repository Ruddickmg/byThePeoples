use crate::{
    configuration::jwt,
    model,
    model::credentials::{CredentialId, Credentials},
    Error,
};
use actix_web::{dev, http, web};
use jsonwebtoken;
use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: CredentialId,
    email: String,
    name: String,
    exp: usize,
}

pub fn generate_token(credentials: Credentials) -> Result<String, Error> {
    let Credentials {
        id, name, email, ..
    } = credentials;
    match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            id,
            name,
            email,
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

pub fn set_auth_header(
    mut response: dev::HttpResponseBuilder,
    credentials: model::Credentials,
) -> Result<web::HttpResponse, Error> {
    let token = generate_token(credentials)?;
    Ok(response.header(http::header::AUTHORIZATION, token).finish())
}
