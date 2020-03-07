// use crate::{
//     authentication::{jwt, password},
//     model::{credentials, AuthRequest, Database},
//     Error,
// };
// use std::sync;
//
// pub async fn authorize<'a>(
//     user_credentials: AuthRequest,
//     mut db: sync::MutexGuard<'a, Database<'a>>,
// ) -> Result<Option<String>, Error> {
//     let client: database::Client<'a> = db.client().await?;
//     let mut auth_credentials: credentials::Model<'a> = credentials::Model::new(client);
//     if let Some(auth_record) = auth_credentials.by_name(&user_credentials.name).await? {
//         if password::authenticate(&user_credentials.password, &auth_record.hash)? {
//             return Ok(Some(String::from(jwt::generate_token(auth_record)?)));
//         }
//     }
//     Ok(None)
// }
