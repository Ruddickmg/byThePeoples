use crate::{
    configuration::jwt,
    model,
    model::credentials::{CredentialId, Credentials},
    error::Error,
    Result,
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

pub fn generate_token(credentials: Credentials) -> Result<String> {
    let Credentials {
        id, name, email, ..
    } = credentials;
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims {
            id,
            name,
            email,
            exp: jwt::expiration(),
        },
        &EncodingKey::from_secret(&jwt::secret().as_ref()),
    )
        .map_err(| error | Error::InternalServerError(error.to_string()))
}

pub fn set_token(
    mut response: dev::HttpResponseBuilder,
    credentials: model::Credentials,
) -> Result<web::HttpResponse> {
    let token = generate_token(credentials)?;
    Ok(response.header(http::header::AUTHORIZATION, token).finish())
}
