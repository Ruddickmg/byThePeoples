use serde::export::Formatter;

pub mod configuration;
pub mod constants;
pub mod controller;
pub mod handler;
pub mod model;
pub mod repository;
pub mod routes;
pub mod utilities;

#[derive(Debug, Clone)]
pub enum Error {
    DatabaseError(database::Error),
    InternalServerError(String),
    Unauthorized(argonautica::Error),
    PasswordError(zxcvbn::ZxcvbnError),
    SystemTimeError(std::time::SystemTimeError),
    BadRequest(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(error) => write!(f, "{}", error),
            Error::InternalServerError(error) => write!(f, "{}", error),
            Error::Unauthorized(error) => write!(f, "{}", error),
            Error::BadRequest(message) => write!(f, "{}", message),
            Error::PasswordError(error) => write!(f, "{}", error),
            Error::SystemTimeError(error) => write!(f, "{}", error),
        }
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Error {
        Error::SystemTimeError(error)
    }
}

impl From<zxcvbn::ZxcvbnError> for Error {
    fn from(error: zxcvbn::ZxcvbnError) -> Error {
        Error::PasswordError(error)
    }
}

impl From<database::Error> for Error {
    fn from(error: database::Error) -> Error {
        Error::DatabaseError(error)
    }
}

impl From<argonautica::Error> for Error {
    fn from(error: argonautica::Error) -> Error {
        Error::Unauthorized(error)
    }
}

pub mod connection {
    const PORT: &str = "PORT";
    const ADDRESS: &str = "IP";
    const DEFAULT_PORT: &str = "8080";
    const ALL_ADDRESSES: &str = "0.0.0.0";
    pub fn uri() -> String {
        let ip = environment::env_or_default(ADDRESS, ALL_ADDRESSES);
        let port = environment::env_or_default(PORT, DEFAULT_PORT);
        format!("{}:{}", ip, port)
    }
}
