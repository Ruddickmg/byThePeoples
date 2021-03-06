use serde::export::Formatter;

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

impl From<ring::error::Unspecified> for Error {
    fn from(error: ring::error::Unspecified) -> Error {
        Error::InternalServerError(error.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Error {
        Error::InternalServerError(error.to_string())
    }
}