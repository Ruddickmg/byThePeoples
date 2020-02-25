mod auth;
mod playground;
use juniper;
use juniper::FieldResult;

pub struct Query;

#[juniper::object]
impl Query {
    fn playground() -> FieldResult<playground::Playground> {
        Ok(playground::Playground)
    }
    fn credentials() -> FieldResult<auth::Auth> {
        Ok(auth::Auth)
    }
}
