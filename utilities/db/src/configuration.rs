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
    pub fn build(&self) -> tokio_postgres::Config {
        let Configuration {
            user,
            host,
            port,
            password,
            database,
        } = self;
        format!(
            "dbname={} host={} password='{}' user={} port={} connect_timeout={}",
            database, host, password, user, port, TIME_OUT
        )
        .parse()
        .unwrap()
    }
}
