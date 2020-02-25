use super::mutations::Mutation;
use super::queries::Query;
use juniper::RootNode;

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
