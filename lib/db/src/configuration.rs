use crate::Error;
use bb8_postgres::tokio_postgres;

pub const POOL_SIZE: u32 = 15;
const TIME_OUT: u32 = 10;

pub struct Configuration {
    pub password: &'static str,
    pub user: &'static str,
    pub host: &'static str,
    pub port: &'static str,
    pub database: &'static str,
}

impl Configuration {
    pub fn build(&self) -> Result<tokio_postgres::Config, Error> {
        let Configuration {
            user,
            host,
            port,
            password,
            database,
        } = self;
        if let Ok(config) = format!(
            "dbname={} host={} password='{}' user={} port={} connect_timeout={}",
            database, host, password, user, port, TIME_OUT
        )
        .parse()
        {
            Ok(config)
        } else {
            Err(Error::from("Failed to build database configuration."))
        }
    }
}

#[cfg(test)]
mod configuration_test {
    use super::*;

    #[test]
    fn credentials_are_built_correctly() {
        let user = "postgres";
        let host = "localhost";
        let port = "8989";
        let password = "secret";
        let database = "auth";
        let config = Configuration {
            user,
            host,
            port,
            password,
            database,
        };
        let built = config.build().unwrap();
        assert_eq!(
            built,
            format!(
                "dbname={} host={} password='{}' user={} port={} connect_timeout={}",
                database, host, password, user, port, TIME_OUT
            )
            .parse()
            .unwrap()
        )
    }
}
