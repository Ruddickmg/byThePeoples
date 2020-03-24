use bb8;
use bb8_postgres;
use chrono;
use std::fmt::Formatter;
use std::{fmt::Display, io};
use tokio_postgres;

mod client;
mod configuration;
mod connection_pool;
mod transaction;

pub type Timestamp = chrono::NaiveDateTime;
pub type Statement = tokio_postgres::Statement;
pub type Results = Vec<tokio_postgres::Row>;
pub type Params<'a> = &'a [&'a (dyn tokio_postgres::types::ToSql + Sync)];
pub type Transaction<'a> = transaction::GenericTransaction<'a>;
pub type Client<'a> = client::GenericClient<'a>;
pub type Configuration = configuration::Configuration;
pub type Database = Box<dyn connection_pool::DatabaseTrait + Send + Sync>;

pub use connection_pool::ConnectionPool;

type Result<T> = std::result::Result<T, Error>;
type Manager = bb8_postgres::PostgresConnectionManager<tokio_postgres::tls::NoTls>;
type PooledConnection<'a> = bb8::PooledConnection<'a, Manager>;
type Pool = bb8::Pool<Manager>;
type ConnectionError = bb8::RunError<tokio_postgres::Error>;

#[derive(Debug)]
pub enum Error {
    DatabaseError(tokio_postgres::Error),
    ConnectionErr(ConnectionError),
    IoError(io::Error),
    Error(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(error) => write!(f, "{}", error),
            Error::Error(error) => write!(f, "{}", error),
            Error::ConnectionErr(error) => write!(f, "{}", error),
            Error::IoError(error) => write!(f, "{}", error),
        }
    }
}

impl From<&str> for Error {
    fn from(string: &str) -> Error {
        Error::Error(String::from(string))
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(error: tokio_postgres::Error) -> Self {
        Error::DatabaseError(error)
    }
}

impl From<ConnectionError> for Error {
    fn from(error: ConnectionError) -> Self {
        Error::ConnectionErr(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
