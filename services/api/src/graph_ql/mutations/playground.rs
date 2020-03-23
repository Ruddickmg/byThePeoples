use super::Mutation;
use crate::models::playground;
use crate::redis;
use juniper::FieldResult;

#[juniper::object]
impl Mutation {
    fn create_human(new_human: playground::NewHuman) -> FieldResult<playground::Human> {
        let human = playground::Human {
            id: "1234".to_owned(),
            name: new_human.name.clone(),
            appears_in: new_human.appears_in.clone(),
            home_planet: new_human.home_planet.clone(),
        };
        redis::cache_human(&human)?;
        Ok(human)
    }
}
