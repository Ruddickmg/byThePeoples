use bb8;
use bb8_postgres;
use std::fmt::Formatter;
use std::time::SystemTime;
use std::{fmt::Display, io};
use tokio_postgres;

mod client;
pub mod configuration;
mod connection_pool;
mod transaction;

pub type Row = tokio_postgres::Row;
pub type Timestamp = SystemTime;
pub type Results = Vec<Row>;
pub type Params<'a> = &'a [&'a (dyn tokio_postgres::types::ToSql + Sync)];
pub type SmallInt = i16;

pub use client::Client as DatabaseClient;
pub use client::ClientTrait as Client;
pub use configuration::Configuration;
pub use connection_pool::ConnectionPool as DatabaseConnection;
pub use connection_pool::ConnectionPoolTrait as Database;
pub use std::time::SystemTime as TimeStamp;
pub use tokio_postgres::Statement;
pub use transaction::Transaction;

type Result<T> = std::result::Result<T, Error>;
type Manager = bb8_postgres::PostgresConnectionManager<tokio_postgres::tls::NoTls>;
type PooledConnection<'a> = bb8::PooledConnection<'a, Manager>;
type Pool = bb8::Pool<Manager>;
type ConnectionError = bb8::RunError<tokio_postgres::Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Error(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Error(error) => write!(f, "{}", error),
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
        Error::Error(error.to_string())
    }
}

impl From<ConnectionError> for Error {
    fn from(error: ConnectionError) -> Self {
        Error::Error(error.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Error(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
