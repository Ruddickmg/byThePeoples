use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum, Clone, Debug)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

impl Episode {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Episode::NewHope => "NewHope",
            Episode::Empire => "Empire",
            Episode::Jedi => "Jedi",
        })
    }
    pub fn from_string(value: &str) -> Result<Episode, String> {
        match value {
            "NewHope" => Ok(Episode::NewHope),
            "Empire" => Ok(Episode::Empire),
            "Jedi" => Ok(Episode::Jedi),
            _ => Err(format!("Received invalid episode: {}", &value)),
        }
    }
}

#[derive(GraphQLObject, Clone, Debug)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    pub id: String,
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}
