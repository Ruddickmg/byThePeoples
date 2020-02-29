use crate::models::playground;
use crate::redis;
use juniper;

pub struct Playground;

#[juniper::object]
impl Playground {
    fn human(id: String) -> juniper::FieldResult<playground::Human> {
        Ok(redis::get_human_from_cache(id)?)
    }
}
