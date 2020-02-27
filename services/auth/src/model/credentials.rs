pub struct Model<'a> {
    db: &'a database::DB,
}

pub struct Credentials {
    pub id: Option<u32>,
    pub email: Option<String>,
    pub name: String,
    pub hash: String,
}

impl Credentials {
    fn from_results(rows: database::Results) -> Option<Credentials> {
        match rows.first() {
            Some(result) => Some(Credentials {
                id: Some(result.get(0)),
                email: Some(result.get(1)),
                name: result.get(2),
                hash: result.get(3),
            }),
            None => None,
        }
    }
}

const GET_CREDENTIALS_BY_NAME: &str =
    "SELECT id, email, name, hash FROM auth.credentials WHERE name = $1";

impl<'a> Model<'a> {
    pub fn new(db: &'a mut database::DB) -> Model<'a> {
        Model { db }
    }
    pub async fn by_name(&mut self, name: &str) -> Result<Credentials, database::DBError> {
        let mut client = self.db.client().await?;
        let statement = client.prepare(GET_CREDENTIALS_BY_NAME).await?;
        if let Ok(rows) = client.query(&statement, &[&name]).await {
            if let Some(credentials) = Credentials::from_results(rows) {
                return Ok(credentials);
            }
        }
        panic!("Nooooo!");
    }
}
