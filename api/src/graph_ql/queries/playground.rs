use super::super::models::playground::{Episode, Human};
use super::Query;
use juniper::FieldResult;

#[juniper::object]
impl Query {
    fn human(id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}
