use crate::Error;

mod client;
mod connection_pool;
mod transaction;

pub use client::MockClient as Client;
pub use connection_pool::MockConnectionPool as ConnectionPool;
pub use transaction::MockTransaction as Transaction;

type MockMethod<T> = mock::Method<T, Error>;
