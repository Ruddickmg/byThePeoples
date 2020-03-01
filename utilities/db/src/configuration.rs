use crate::Error;
use bb8_postgres::tokio_postgres;

pub const POOL_SIZE: u32 = 15;
const TIME_OUT: u32 = 10;

pub struct Configuration {
    pub password: String,
    pub user: String,
    pub host: String,
    pub port: String,
    pub database: String,
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
            user: String::from(user),
            host: String::from(host),
            port: String::from(port),
            password: String::from(password),
            database: String::from(database),
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
