use super::Mutation;
use crate::graph_ql::models::playground::{Human, NewHuman};
use juniper::FieldResult;

#[juniper::object]
impl Mutation {
    fn createHuman(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}
