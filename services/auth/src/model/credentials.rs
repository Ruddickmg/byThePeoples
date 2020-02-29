pub struct Credentials {
    pub id: u32,
    pub email: String,
    pub name: String,
    pub hash: String,
}

impl Credentials {
    fn from_results(rows: database::Results) -> Option<Credentials> {
        match rows.first() {
            Some(result) => Some(Credentials {
                id: result.get(0),
                email: result.get(1),
                name: result.get(2),
                hash: result.get(3),
            }),
            None => None,
        }
    }
}

const GET_CREDENTIALS_BY_NAME: &str =
    "SELECT id, email, name, hash FROM auth.credentials WHERE name = $1";

pub struct Model<'a> {
    client: database::Client<'a>,
}

impl<'a> Model<'a> {
    pub fn new(client: database::Client<'a>) -> Model {
        Model { client }
    }
    pub async fn by_name(&mut self, name: &str) -> Result<Option<Credentials>, database::Error> {
        let statement = self.client.prepare(GET_CREDENTIALS_BY_NAME).await?;
        let rows = self.client.query(&statement, &[&name]).await?;
        Ok(Credentials::from_results(rows))
    }
}
