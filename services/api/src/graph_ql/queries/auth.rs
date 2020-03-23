use crate::models::auth;
use juniper::FieldResult;

pub struct Auth;

#[juniper::object]
impl Auth {
    fn get_credentials(_username: String) -> FieldResult<auth::Credentials> {
        println!("getting credentials");
        Ok(auth::Credentials {
            username: "testing".to_owned(),
            password: "hashedPassword".to_owned(),
        })
    }
}
