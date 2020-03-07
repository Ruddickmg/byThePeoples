// use crate::{client::Client, Error, Manager};
// use bb8;
//
// #[async_trait]
// pub trait DatabaseTrait {
//     async fn client(&mut self) -> Result<Client<'_>, Error>;
//     async fn migrate(&mut self, path: &str) -> Result<(), Error>;
// }
//
// #[async_trait]
// impl DatabaseTrait for Connection {
//     async fn client(&mut self) -> Result<Client<'_>, Error> {
//         self.client().await
//     }
//     async fn migrate(&mut self, path: &str) -> Result<(), Error> {
//         self.migrate(path).await
//     }
// }
//
// #[derive(Clone)]
// pub struct Connection<'a> {
//     connection: bb8::PooledConnection<'a, Manager>,
// }
//
// impl<'a> Connection<'a> {
//     pub async fn new(connection: bb8::PooledConnection<'a, Manager>) -> Result<DB, Error> {
//         Ok(Connection { connection })
//     }
//     pub async fn client(&'a mut self) -> Result<Client<'a>, Error> {
//         Ok(Client::new(self.connection.))
//     }
//     // pub async fn migrate<'a>(&mut self, path: &str) -> Result<(), Error> {
//     //     let sql_files = files::by_extension(path, SQL_EXTENSION);
//     //     let pool = self.connection.get();
//     //     let mut client = pool.get().await?;
//     //     let transaction: tokio_postgres::Transaction<'a> = client.transaction().await?;
//     //     for file_path in sql_files.iter() {
//     //         let sql = fs::read_to_string(file_path)?;
//     //         transaction.batch_execute(&sql).await?;
//     //     }
//     //     Ok(transaction.commit().await?)
//     // }
// }
