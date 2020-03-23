use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[graphql(
    description = "Stored credentials (hashed password and user name) for the specified user"
)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
