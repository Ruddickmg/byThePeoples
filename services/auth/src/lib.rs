use serde::export::Formatter;

mod configuration;
pub mod controller;
mod handler;
pub mod model;
mod repository;
pub mod routes;
mod utilities;

#[derive(Debug)]
pub enum InternalServerError {
    Unknown(String),
    Actix(actix_web::Error),
}

impl std::fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalServerError::Unknown(error) => write!(f, "{}", error),
            InternalServerError::Actix(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    DatabaseError(database::Error),
    InternalServerError(InternalServerError),
    Unauthorized(argonautica::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(error) => write!(f, "{}", error),
            Error::InternalServerError(error) => write!(f, "{}", error),
            Error::Unauthorized(error) => write!(f, "{}", error),
        }
    }
}

impl From<database::Error> for Error {
    fn from(error: database::Error) -> Error {
        Error::DatabaseError(error)
    }
}

impl From<actix_web::error::Error> for Error {
    fn from(error: actix_web::error::Error) -> Error {
        Error::InternalServerError(InternalServerError::Actix(error))
    }
}

impl From<argonautica::Error> for Error {
    fn from(error: argonautica::Error) -> Error {
        Error::Unauthorized(error)
    }
}

pub mod logging {
    pub fn log_error(error: String) {
        println!("Error occurred: {}", error);
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
