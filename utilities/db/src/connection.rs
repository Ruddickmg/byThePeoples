use crate::{configuration, Error};
use bb8;
use bb8_postgres;
use tokio_postgres;

pub type Manager = bb8_postgres::PostgresConnectionManager<tokio_postgres::tls::NoTls>;
pub type Pool = bb8::Pool<Manager>;
pub type Client<'a> = bb8::PooledConnection<'a, Manager>;

pub struct Connection {
    pool: Pool,
}

impl<'a> Connection {
    pub async fn new(
        cfg: configuration::Configuration,
    ) -> Result<Connection, tokio_postgres::Error> {
        let manager =
            bb8_postgres::PostgresConnectionManager::new(cfg.build(), tokio_postgres::tls::NoTls);
        let pool: Pool = bb8::Pool::builder()
            .max_size(configuration::POOL_SIZE)
            .build(manager)
            .await?;
        Ok(Connection { pool })
    }
    pub async fn get(&'a mut self) -> Result<Client<'a>, Error> {
        self.pool = self.pool.clone();
        Ok(self.pool.get().await?)
    }
}
