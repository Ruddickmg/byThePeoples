use crate::configuration;
use bb8;
use bb8_postgres;
use tokio_postgres;

pub type Manager = bb8_postgres::PostgresConnectionManager<tokio_postgres::tls::NoTls>;
pub type Pool = bb8::Pool<Manager>;
pub type Connection<'a> = bb8::PooledConnection<'a, Manager>;
pub type Error = bb8::RunError<tokio_postgres::Error>;

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool,
}

impl<'a> ConnectionPool {
    pub async fn new(cfg: configuration::Configuration) -> Result<ConnectionPool, Error> {
        let manager =
            bb8_postgres::PostgresConnectionManager::new(cfg.build(), tokio_postgres::tls::NoTls);
        let pool: Pool = bb8::Pool::builder()
            .max_size(configuration::POOL_SIZE)
            .build(manager)
            .await?;
        Ok(ConnectionPool { pool })
    }
    pub async fn get(&'a mut self) -> Result<Connection<'a>, Error> {
        self.pool = self.pool.clone();
        Ok(self.pool.get().await?)
    }
}
